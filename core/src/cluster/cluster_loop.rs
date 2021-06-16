use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};

use crate::broker::{Alive, Broker, Initializing};
use kafka_connector_protocol::{
    api::metadata::{MetadataRequest, MetadataRequestTopics0},
    custom_types::tag_buffer::TagBuffer,
};
use log::trace;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use super::{
    metadata::{Metadata, PartitionMetadata, TopicMetadata},
    ClusterInner,
};

#[derive(Debug)]
pub(crate) enum ClusterLoopSignal {
    /// On broker successfully connecting
    BrokerConnected(i32, Broker<Alive>),
    /// On broker failing to connect/disconnected
    BrokerDisconnected(i32),
    /// Timed event to refresh metadata from time to time
    RefreshMetadataRequest,
    /// Up to date metadata
    RefreshMetadataResponse(Option<Metadata>),
    /// Disconnect from kafka brokers, clean up
    Shutdown,
}

#[derive(Debug)]
pub(crate) enum BrokerState {
    Alive(Broker<Alive>),
    Initializing(Broker<Initializing>),
}

pub(super) async fn cluster_loop(
    clients: HashMap<i32, SocketAddr>,
    loop_signal_receiver: UnboundedReceiver<ClusterLoopSignal>,
    cluster: Arc<ClusterInner>,
) {
    {
        let mut brokers = cluster.brokers.write().await;
        for client in clients {
            let broker = Broker::new(client.1, cluster.options.clone());
            broker.new_no_wait(cluster.loop_signal_sender.clone(), client.0);
            brokers.insert(client.0, BrokerState::Initializing(broker));
        }
    }

    let metadata_refresh_stream =
        futures::stream::repeat_with(|| ClusterLoopSignal::RefreshMetadataRequest)
            .throttle(cluster.options.metadata_refresh_timeout);

    let mut stream =
        Box::pin(metadata_refresh_stream.merge(UnboundedReceiverStream::new(loop_signal_receiver)));

    while let Some(signal) = stream.next().await {
        match signal {
            ClusterLoopSignal::BrokerConnected(broker_id, broker) => {
                let mut brokers = cluster.brokers.write().await;
                let old_state = brokers.remove(&broker_id).expect("Unknown broker id");
                let new_state = if let BrokerState::Initializing(_addr) = old_state {
                    BrokerState::Alive(broker)
                } else {
                    panic!("Wrong broker state")
                };
                brokers.insert(broker_id, new_state);
            }
            ClusterLoopSignal::BrokerDisconnected(broker_id) => {
                let mut brokers = cluster.brokers.write().await;
                let old_state = brokers.remove(&broker_id).expect("Unknown broker id");
                let new_state = if let BrokerState::Alive(old_broker) = old_state {
                    let broker = Broker::new(old_broker.addr, old_broker.options);
                    broker.new_no_wait(cluster.loop_signal_sender.clone(), broker_id);
                    BrokerState::Initializing(broker)
                } else {
                    panic!("Wrong broker state")
                };
                brokers.insert(broker_id, new_state);

                // TODO: some waiting,circuit breaker or sth?
                // TODO: restart only when broker is not deleted(on metadata update)
            }
            ClusterLoopSignal::RefreshMetadataRequest => {
                let mut brokers = cluster.brokers.write().await;
                if let Some((_, BrokerState::Alive(broker))) = brokers
                    .iter_mut()
                    .find(|broker| matches!(broker.1, BrokerState::Alive { .. }))
                {
                    // TODO: Change to non-blocking
                    refresh_cluster_metadata(
                        broker,
                        cluster.metadata.clone(),
                        cluster.loop_signal_sender.clone(),
                    )
                    .await;
                };
            }
            ClusterLoopSignal::RefreshMetadataResponse(new_metadata) => {
                let new_metadata = if let Some(new_metadata) = new_metadata {
                    new_metadata
                } else {
                    todo!("should retry when broker gets close")
                };
                let mut brokers_to_start = HashMap::new();
                {
                    let mut metadata = cluster.metadata.write().await;

                    let mut brokers_to_delete = vec![];
                    // Removed
                    for broker_id in metadata.brokers.keys() {
                        if !new_metadata.brokers.contains_key(broker_id) {
                            brokers_to_delete.push(*broker_id);
                        }
                    }
                    for broker_id in brokers_to_delete {
                        metadata.brokers.remove(&broker_id);
                    }

                    // Changed and added
                    for broker in new_metadata.brokers {
                        match metadata.brokers.get(&broker.0) {
                            Some(old) => {
                                if old != &broker.1 {
                                    // Change
                                    panic!("Broker metadata change while client is active");
                                }
                            }
                            None => {
                                brokers_to_start.insert(broker.0, broker.1.clone());
                                metadata.brokers.insert(broker.0, broker.1);
                            }
                        }
                    }

                    for topic in new_metadata.topics {
                        metadata.topics.insert(topic.0, topic.1);
                    }
                }
                {
                    let mut brokers = cluster.brokers.write().await;

                    for broker_data in brokers_to_start {
                        let addr = (broker_data.1 .0.as_str(), broker_data.1 .1 as u16)
                            .to_socket_addrs()
                            .unwrap()
                            .next()
                            .unwrap(); // TODO: Remove unwraps

                        let broker = Broker::new(addr, cluster.options.clone());
                        broker.new_no_wait(cluster.loop_signal_sender.clone(), broker_data.0);
                        brokers.insert(broker_data.0, BrokerState::Initializing(broker));
                    }
                }
            }
            ClusterLoopSignal::Shutdown => {
                break;
            }
        }
    }
    // TODO: Should first try to gracefully close all producer/consumers?
    let mut brokers = cluster.brokers.write().await;
    brokers.clear();
    trace!("Cluster loop close")
}

pub(crate) async fn refresh_cluster_metadata(
    broker: &mut Broker<Alive>,
    metadata: Arc<RwLock<Metadata>>,
    sender: UnboundedSender<ClusterLoopSignal>,
) {
    let topics = {
        let metadata = metadata.read().await;
        metadata
            .topics
            .keys()
            .map(|topic_name| MetadataRequestTopics0 {
                name: topic_name.clone(),
                tag_buffer: TagBuffer {},
            })
            .collect()
    };
    let request = MetadataRequest {
        topics,
        allow_auto_topic_creation: false,
        include_cluster_authorized_operations: false,
        include_topic_authorized_operations: false,
        tag_buffer: TagBuffer {},
    };
    let response = broker.run_api_call_with_retry(request, None).await;
    let response = if let Ok(response) = response {
        response
    } else {
        if sender
            .send(ClusterLoopSignal::RefreshMetadataResponse(None))
            .is_err()
        {
            log::debug!("Cannot refresh metadata - cluster loop already dropped")
        };
        return;
    };

    let brokers = response
        .brokers
        .into_iter()
        .map(|broker| (broker.node_id, (broker.host, broker.port)))
        .collect();
    let topics = response
        .topics
        .into_iter()
        .map(|topic| {
            (
                topic.name,
                TopicMetadata {
                    is_internal: topic.is_internal.unwrap_or_default(),
                    partitions: topic
                        .partitions
                        .into_iter()
                        .map(|topic_partition| {
                            (
                                topic_partition.partition_index,
                                PartitionMetadata {
                                    leader_id: topic_partition.leader_id,
                                    leader_epoch: topic_partition.leader_epoch,
                                    replica_nodes: topic_partition.replica_nodes,
                                    isr_nodes: topic_partition.isr_nodes,
                                    offline_replicas: topic_partition.offline_replicas,
                                },
                            )
                        })
                        .collect(),
                },
            )
        })
        .collect();
    let metadata = Metadata { brokers, topics };
    if sender
        .send(ClusterLoopSignal::RefreshMetadataResponse(Some(metadata)))
        .is_err()
    {
        log::debug!("Cannot refresh metadata - cluster loop already dropped")
    };
}
