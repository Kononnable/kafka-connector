use std::sync::Arc;

use kafka_connector_protocol::api::metadata::{MetadataRequest, MetadataRequestTopics0};
use log::{debug, trace};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    oneshot::{self, Receiver},
};

use crate::{
    cluster::{
        cluster_loop::BrokerState,
        metadata::{PartitionMetadata, TopicMetadata},
        Cluster,
    },
    producer::producer_loop::ProducerLoopSignal,
};

use self::{options::ProducerOptions, record::ProducerRecord};

pub mod error;
pub mod options;
mod producer_loop;
pub mod record;
pub mod send_result_future;

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
    pub async fn send(&mut self, record: ProducerRecord) -> SendResultFuture2 {
        // TODO: Unwraps
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
            let mut brokers = self.cluster.inner.brokers.write().await;
            if let Some((_, BrokerState::Alive { broker, .. })) = brokers
                .iter_mut()
                .find(|broker| matches!(broker.1, BrokerState::Alive { .. }))
            {
                let response = broker
                    .run_api_call_with_retry(
                        MetadataRequest {
                            allow_auto_topic_creation: true, // TODO: Config options
                            topics: vec![MetadataRequestTopics0 {
                                name: record.topic.clone(),
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
                    .find(|x| x.name == record.topic)
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
                )
            } else {
                todo!("what to do if not connected yet")
            };
        }

        let (sender, receiver) = oneshot::channel();
        self.loop_signal_sender
            .send(ProducerLoopSignal::SendMessage(record, sender))
            .unwrap();
        receiver
    }
}

type SendResultFuture2 = Receiver<()>;

impl Drop for Producer {
    fn drop(&mut self) {
        debug!("Producer is being dropped");
        self.loop_signal_sender
            .send(ProducerLoopSignal::Shutdown)
            .expect("Producer loop should be alive.")
    }
}
