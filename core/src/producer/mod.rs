use std::sync::Arc;

use kafka_connector_protocol::api::metadata::{MetadataRequest, MetadataRequestTopics0};
use log::{debug, error, trace};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    oneshot::{self, Receiver},
};

use crate::{
    broker::{Alive, Broker},
    cluster::{
        cluster_loop::BrokerState,
        metadata::{PartitionMetadata, TopicMetadata},
        Cluster,
    },
    producer::producer_loop::ProducerLoopSignal,
};

use self::{options::ProducerOptions, record::ProducerRecord};

pub mod options;
mod producer_loop;
pub mod record;

pub struct Producer {
    loop_signal_sender: UnboundedSender<ProducerLoopSignal>,
    cluster: Arc<Cluster>,
    options: ProducerOptions,
}

impl Producer {
    /// Creates kafka producer and waits for it to connect to at least one broker
    pub async fn new(cluster: Arc<Cluster>, options: ProducerOptions) -> Self {
        let (loop_signal_sender, loop_signal_receiver) = unbounded_channel();

        tokio::spawn(producer_loop::producer_loop(
            loop_signal_receiver,
            cluster.inner.clone(),
        ));
        Producer {
            cluster,
            options,
            loop_signal_sender,
        }
    }
    pub async fn send(&mut self, record: ProducerRecord) -> SendFuture {
        trace!("Send start");

        if !self
            .cluster
            .inner
            .metadata
            .read()
            .await
            .topics
            .contains_key(&record.topic)
        {
            trace!("Fetching topic metadata start");
            //TODO: Extract looking for a connected broker to a function/sending request to any broker
            let mut brokers = self.cluster.inner.brokers.write().await;
            if let Some((_, BrokerState::Alive(broker))) = brokers
                .iter_mut()
                .find(|broker| matches!(broker.1, BrokerState::Alive { .. }))
            {
                self.fetch_topic_metadata(broker, &record.topic).await;
            } else {
                todo!("what to do if not connected yet")
            };
        }

        let (sender, receiver) = oneshot::channel();
        self.loop_signal_sender
            .send(ProducerLoopSignal::SendMessage(record, sender))
            .expect("Producer loop dead");
        receiver
    }

    async fn fetch_topic_metadata<S>(&self, broker: &mut Broker<Alive>, topic_name: S)
    where
        S: Into<String>,
    {
        // TODO: Unwraps
        let topic_name = topic_name.into();
        let response = broker
            .run_api_call_with_retry(
                MetadataRequest {
                    allow_auto_topic_creation: self.options.allow_auto_topic_creation,
                    topics: vec![MetadataRequestTopics0 {
                        name: topic_name.clone(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();
        let topic = response
            .topics
            .into_iter()
            .find(|x| x.name == topic_name)
            .unwrap();
        let mut metadata = self.cluster.inner.metadata.write().await;
        metadata.topics.insert(
            topic.name,
            TopicMetadata {
                is_internal: topic.is_internal.unwrap_or_default(),
                partitions: topic
                    .partitions
                    .into_iter()
                    .map(|x| {
                        (
                            x.partition_index,
                            PartitionMetadata {
                                isr_nodes: x.isr_nodes,
                                leader_epoch: x.leader_epoch,
                                leader_id: x.leader_id,
                                offline_replicas: x.offline_replicas,
                                replica_nodes: x.replica_nodes,
                            },
                        )
                    })
                    .collect(),
            },
        );
    }
}

type SendFuture = Receiver<()>;

impl Drop for Producer {
    fn drop(&mut self) {
        debug!("Producer is being dropped");
        let result = self.loop_signal_sender.send(ProducerLoopSignal::Shutdown);

        if result.is_err() {
            error!("Producer dropped when loop is already dead");
        }
    }
}
