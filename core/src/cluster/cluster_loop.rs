use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
    time::Duration,
};

use crate::broker::Broker;
use log::trace;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use super::{metadata::Metadata, ClusterInner};

#[derive(Debug)]
pub(crate) enum ClusterLoopSignal {
    /// On broker successfully connecting
    BrokerConnected(i32, Broker),
    /// On broker failing to connect/disconnected
    BrokerDisconnected(i32),
    /// Timed event to refresh metadata from time to time
    RefreshMetadataRequest,
    /// Up to date metadata
    RefreshMetadataResponse(Option<Metadata>),
    /// Disconnect from kafka brokers, clean up
    Shutdown,
}

// TODO: Use typestate pattern?
#[derive(Debug)]
pub(crate) enum BrokerState {
    Alive { broker: Broker, addr: SocketAddr },
    Initializing { addr: SocketAddr },
}

pub(super) async fn cluster_loop(
    clients: HashMap<i32, SocketAddr>,
    loop_signal_receiver: UnboundedReceiver<ClusterLoopSignal>,
    cluster: Arc<ClusterInner>,
) {
    {
        let mut brokers = cluster.brokers.write().await;
        for client in clients {
            brokers.insert(client.0, BrokerState::Initializing { addr: client.1 });
            Broker::new_no_wait(
                client.1,
                cluster.options.clone(),
                cluster.loop_signal_sender.clone(),
                client.0,
            );
        }
    }

    let metadata_refresh_stream =
        futures::stream::repeat_with(|| ClusterLoopSignal::RefreshMetadataRequest)
            .throttle(Duration::from_secs(180)); // TODO: configurable?,  change value

    let mut stream =
        Box::pin(metadata_refresh_stream.merge(UnboundedReceiverStream::new(loop_signal_receiver)));

    while let Some(signal) = stream.next().await {
        match signal {
            ClusterLoopSignal::BrokerConnected(broker_id, broker) => {
                let mut brokers = cluster.brokers.write().await;
                let old_state = brokers.remove(&broker_id).expect("Unknown broker id");
                let new_state = if let BrokerState::Initializing { addr } = old_state {
                    BrokerState::Alive { addr, broker }
                } else {
                    panic!("Wrong broker state")
                };
                brokers.insert(broker_id, new_state);
            }
            ClusterLoopSignal::BrokerDisconnected(broker_id) => {
                let mut brokers = cluster.brokers.write().await;
                let old_state = brokers.remove(&broker_id).expect("Unknown broker id");
                let new_state = if let BrokerState::Alive { addr, .. } = old_state {
                    Broker::new_no_wait(
                        addr,
                        cluster.options.clone(),
                        cluster.loop_signal_sender.clone(),
                        broker_id,
                    );
                    BrokerState::Initializing { addr }
                } else {
                    panic!("Wrong broker state")
                };
                brokers.insert(broker_id, new_state);

                // TODO: some waiting,circuit breaker or sth?
                // TODO: restart only when broker is not deleted(on metadata update)
            }
            ClusterLoopSignal::RefreshMetadataRequest => {
                let mut brokers = cluster.brokers.write().await;
                if let Some((_, BrokerState::Alive { broker, .. })) = brokers
                    .iter_mut()
                    .find(|broker| matches!(broker.1, BrokerState::Alive { .. }))
                {
                    // TODO: Change to non-blocking
                    broker
                        .refresh_cluster_metadata(
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

                    for broker in brokers_to_start {
                        let addr = (broker.1 .0.as_str(), broker.1 .1 as u16)
                            .to_socket_addrs()
                            .unwrap()
                            .next()
                            .unwrap(); // TODO: Remove unwraps

                        brokers.insert(broker.0, BrokerState::Initializing { addr });
                        Broker::new_no_wait(
                            addr,
                            cluster.options.clone(),
                            cluster.loop_signal_sender.clone(),
                            broker.0,
                        );
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
