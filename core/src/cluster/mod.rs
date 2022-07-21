pub(crate) mod cluster_loop;
pub mod error;
pub mod metadata;

use std::{collections::HashMap, net::ToSocketAddrs, sync::Arc, time::Duration};

use futures_util::future::select_all;
use kafka_connector_protocol::{
    api::metadata::{MetadataRequest, MetadataRequestTopics0, MetadataResponse0},
    ApiCall,
};
use log::{debug, error, warn};
use tokio::{
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        RwLock,
    },
    time::sleep,
};

use crate::{
    broker::{error::KafkaApiCallError, options::KafkaClientOptions, Broker},
    cluster::metadata::{PartitionMetadata, TopicMetadata},
};

use self::{
    cluster_loop::{BrokerState, ClusterLoopSignal},
    error::ClusterClientCreationError,
    metadata::Metadata,
};

#[derive(Debug)]
pub(crate) struct ClusterInner {
    pub brokers: RwLock<HashMap<i32, BrokerState>>,
    pub metadata: Arc<RwLock<Metadata>>,
    pub client_options: Arc<KafkaClientOptions>,
    pub loop_signal_sender: UnboundedSender<ClusterLoopSignal>,
}

#[derive(Debug)]
pub struct Cluster {
    pub(crate) inner: Arc<ClusterInner>,
}

impl Cluster {
    pub async fn new(
        broker_addrs: impl ToSocketAddrs,
        options: KafkaClientOptions,
    ) -> Result<Cluster, ClusterClientCreationError> {
        let addresses: Vec<_> = broker_addrs
            .to_socket_addrs()
            .map_err(ClusterClientCreationError::AddressRecognitionFailed)?
            .collect();
        if addresses.is_empty() {
            return Err(ClusterClientCreationError::NoClusterAddressFound {});
        }
        let options = Arc::new(options);

        let futures = addresses.into_iter().map(|addr| {
            let broker = Broker::new(addr, options.clone());
            Box::pin(async move {
                let mut client = broker.new_wait().await?;
                let metadata_request = MetadataRequest::default();
                let metadata = client.run_api_call(metadata_request, None).await?;
                Ok(metadata)
            })
        });
        let mut future = select_all(futures);
        let mut errors: Vec<_> = Vec::new();

        let metadata: MetadataResponse0 = loop {
            let (result, _, remaining) = future.await;
            match result {
                Ok(metadata) => {
                    break metadata;
                }
                Err(err) => errors.push(err),
            }
            if remaining.is_empty() {
                return Err(ClusterClientCreationError::ConnectionError(errors));
            }
            future = select_all(remaining);
        };
        let clients = metadata
            .brokers
            .iter()
            .filter_map(|broker| {
                let addr = (broker.host.as_str(), broker.port as u16)
                    .to_socket_addrs()
                    .map_err(|err| {
                        warn!(
                            "Failed to resolve node {} hostname {} {}",
                            broker.node_id, broker.host, err
                        );
                        err
                    })
                    .ok()?
                    .next()?;
                Some((broker.node_id, addr))
            })
            .collect();

        let (loop_signal_sender, loop_signal_receiver) = unbounded_channel();
        let cluster = Cluster {
            inner: Arc::new(ClusterInner {
                metadata: Default::default(),
                client_options: options,
                loop_signal_sender,
                brokers: Default::default(),
            }),
        };

        tokio::spawn(cluster_loop::cluster_loop(
            clients,
            loop_signal_receiver,
            cluster.inner.clone(),
        ));

        Ok(cluster)
    }
}
impl ClusterInner {
    pub async fn fetch_topic_metadata(&self, topics: Vec<String>) -> Result<(), KafkaApiCallError> {
        let response = self
            .run_api_call_on_any_broker(
                MetadataRequest {
                    allow_auto_topic_creation: self.client_options.allow_auto_topic_creation,
                    topics: topics
                        .into_iter()
                        .map(|topic_name| MetadataRequestTopics0 {
                            name: topic_name,
                            ..Default::default()
                        })
                        .collect(),
                    ..Default::default()
                },
                None,
            )
            .await?;

        let mut metadata = self.metadata.write().await;
        response.topics.into_iter().for_each(|topic| {
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
        });
        // TODO: Handle partial errors(topics, partitions)

        Ok(())
    }

    pub async fn run_api_call_on_specific_broker<T>(
        &self,
        broker_id: i32,
        request: T,
        api_version: Option<u16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall,
    {
        // TODO: Timeout or retry count
        let mut brokers = self.brokers.write().await;
        let broker = loop {
            if let Some(BrokerState::Alive(broker)) = brokers.get_mut(&broker_id) {
                break broker;
            };

            drop(brokers);
            debug!("Broker id {} not active", broker_id);
            sleep(Duration::from_millis(100)).await;
            brokers = self.brokers.write().await;
        };

        broker.run_api_call(request, api_version).await
    }
    pub async fn run_api_call_on_any_broker<T>(
        &self,
        request: T,
        api_version: Option<u16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall,
    {
        // TODO: Timeout or retry count
        let mut brokers = self.brokers.write().await;
        let broker = loop {
            if let Some(broker) = brokers.iter_mut().find_map(|x| match x.1 {
                BrokerState::Alive(broker) => Some(broker),
                _ => None,
            }) {
                break broker;
            }
            drop(brokers);
            debug!("No active kafka broker found");
            sleep(Duration::from_millis(100)).await;
            brokers = self.brokers.write().await;
        };

        broker.run_api_call(request, api_version).await
    }
}

impl Drop for Cluster {
    fn drop(&mut self) {
        debug!("Cluster is being dropped, closing all kafka connections");
        let result = self
            .inner
            .loop_signal_sender
            .send(ClusterLoopSignal::Shutdown);

        if result.is_err() {
            error!("Cluster dropped when loop is already dead");
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    const BROKER: &str = "127.0.0.1:9092";
    const CLIENT_ID: &str = "kafka-connector-test";

    fn get_test_client_options() -> KafkaClientOptions {
        KafkaClientOptions::builder()
            .client_id(CLIENT_ID.to_owned())
            .build()
    }

    #[tokio::test]
    async fn should_connect_to_kafka() -> Result<()> {
        env_logger::init();
        Cluster::new(BROKER, get_test_client_options()).await?;
        Ok(())
    }

    // #[tokio::test]
    // async fn should_produce_and_consume_single_message() -> Result<()> {
    //     // TODO: Change to a proper tests
    //     let cluster = Arc::new(Cluster::new(BROKER, get_test_client_options()).await?);

    //     sleep(Duration::from_secs(2)).await;

    //     let mut producer = producer::Producer::new(cluster.clone(), Default::default()).await;
    //     let consumer = consumer::Consumer::new(
    //         cluster,
    //         ConsumerOptions {
    //             group_id: "kafka-connector-test".to_owned(),
    //             topics: vec!["kafka-connector-test".to_owned()],
    //             client_rack: todo!(),
    //             fetch_max_bytes: todo!(),
    //             fetch_max_wait_ms: todo!(),
    //             fetch_min_bytes: todo!(),
    //             heartbeat_interval_ms: todo!(),
    //             max_partition_fetch_bytes: todo!(),
    //         },
    //     )
    //     .await;

    //     sleep(Duration::from_secs(2)).await;

    //     producer
    //         .send(
    //             ProducerRecord::builder()
    //                 .topic("kafka-connector-test".to_owned())
    //                 .payload(b"kafka-connector-test".to_vec())
    //                 .build(),
    //         )
    //         .await;

    //     sleep(Duration::from_secs(3)).await;

    //     let stream = consumer.stream().await.unwrap();
    //     pin!(stream);

    //     stream.try_next().await.unwrap();

    //     Ok(())
    // }
}
