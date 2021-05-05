use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use crate::broker::Broker;
use log::trace;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use super::{metadata::Metadata, Cluster};

#[derive(Debug)]
pub(crate) enum ClusterLoopSignal {
    /// On broker successfully connecting
    BrokerConnected(i32, Broker),
    /// On broker failing to connect/disconnected
    BrokerDisconnected(i32),
    /// Timed event to refresh metadata from time to time
    RefreshMetadataRequest,
    /// Up to date metadata
    RefreshMetadataResponse(Metadata),
    /// Disconnect from kafka brokers, clean up
    Shutdown,
}

#[derive(Debug)]
enum BrokerState {
    Alive { broker: Broker, addr: SocketAddr },
    Initializing { addr: SocketAddr },
}

pub(super) async fn cluster_loop(
    clients: HashMap<i32, SocketAddr>,
    loop_signal_receiver: UnboundedReceiver<ClusterLoopSignal>,
    cluster: Arc<Cluster>,
) {
    let mut brokers: HashMap<_, _> = HashMap::new();
    for client in clients {
        brokers.insert(client.0, BrokerState::Initializing { addr: client.1 });
        Broker::new_no_wait(
            client.1,
            cluster.options.clone(),
            cluster.loop_signal_sender.clone(),
            client.0,
        );
    }

    let metadata_refresh_stream =
        futures::stream::repeat_with(|| ClusterLoopSignal::RefreshMetadataRequest)
            .throttle(Duration::from_secs(2)); // TODO: configurable?,  change value

    let mut stream =
        Box::pin(metadata_refresh_stream.merge(UnboundedReceiverStream::new(loop_signal_receiver)));

    while let Some(signal) = stream.next().await {
        match signal {
            ClusterLoopSignal::BrokerConnected(broker_id, broker) => {
                let old_state = brokers.remove(&broker_id).expect("Unknown broker id");
                let new_state = if let BrokerState::Initializing { addr } = old_state {
                    BrokerState::Alive { addr, broker }
                } else {
                    panic!("Wrong broker state")
                };
                brokers.insert(broker_id, new_state);
            }
            ClusterLoopSignal::BrokerDisconnected(broker_id) => {
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
                let mut metadata = cluster.metadata.write().await;

                // Removed
                for broker_id in metadata.brokers.keys() {
                    if !new_metadata.brokers.contains_key(broker_id) {
                        todo!()
                    }
                }
                // Changed and added
                for broker in new_metadata.brokers {
                    match metadata.brokers.get(&broker.0) {
                        Some(old) => {
                            if old != &broker.1 {
                                // Change
                                todo!()
                            }
                        }
                        None => {
                            metadata.brokers.insert(broker.0, broker.1);
                            // Add
                            todo!()
                        }
                    }
                }

                for topic in new_metadata.topics {
                    metadata.topics.insert(topic.0, topic.1);
                }
            }
            ClusterLoopSignal::Shutdown => {
                break;
            }
        }
    }
    // TODO: close broker connections
    trace!("Cluster loop close")
}
