use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use crate::broker::Broker;
use log::{debug, trace};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use super::{metadata::Metadata, Cluster};

#[derive(Debug, Clone)]
pub(super) enum ClusterLoopSignal {
    /// On broker successfully connecting
    BrokerConnected(i32),
    /// On broker failing to connect/disconnected
    BrokerDisconnected(i32),
    /// Timed event to refresh metadata from time to time
    RefreshMetadataRequest,
    /// Up to date metadata
    RefreshMetadataResponse(Metadata),
    /// Disconnect from kafka brokers, clean up
    Shutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrokerConnection {
    Alive,
    Initializing,
    Dead,
}

struct BrokerState {
    pub broker: Broker,
    pub status: BrokerConnection,
}
impl BrokerState {
    pub fn new(broker: Broker) -> BrokerState {
        BrokerState {
            broker,
            status: BrokerConnection::Dead,
        }
    }
}

pub(super) async fn cluster_loop(
    clients: HashMap<i32, SocketAddr>,
    loop_signal_receiver: UnboundedReceiver<ClusterLoopSignal>,
    cluster: Arc<Cluster>,
) {
    let mut brokers: HashMap<_, _> = clients
        .into_iter()
        .map(|x| {
            (
                x.0,
                BrokerState::new(Broker::new(x.1, cluster.options.clone())),
            )
        })
        .collect();

    for broker in brokers.values_mut() {
        broker.broker.connect(false).await.unwrap(); // when wait = false always Ok(())
        broker.status = BrokerConnection::Initializing;
    }

    let metadata_refresh_stream =
        futures::stream::repeat(ClusterLoopSignal::RefreshMetadataRequest)
            .throttle(Duration::from_secs(2)); // TODO: configurable?,  change value

    let mut stream =
        Box::pin(metadata_refresh_stream.merge(UnboundedReceiverStream::new(loop_signal_receiver)));

    while let Some(signal) = stream.next().await {
        match signal {
            ClusterLoopSignal::BrokerConnected(broker_id) => {
                brokers
                    .get_mut(&broker_id)
                    .expect("Unknown broker id")
                    .status = BrokerConnection::Alive;
            }
            ClusterLoopSignal::BrokerDisconnected(broker_id) => {
                let broker = brokers.get_mut(&broker_id).expect("Unknown broker id");
                broker.status = BrokerConnection::Dead;
                // TODO: should restart the connection automatically?
                // some waiting,circuit breaker or sth?
                broker.status = BrokerConnection::Initializing;
                broker.broker.connect(false).await.unwrap(); // when wait = false always Ok(())
            }
            ClusterLoopSignal::RefreshMetadataRequest => {
                let alive_broker = brokers
                    .iter()
                    .find(|broker| broker.1.status == BrokerConnection::Alive);
                if let Some(broker) = alive_broker {
                    broker
                        .1
                        .broker
                        .refresh_cluster_metadata(cluster.metadata.clone());
                }
            }
            ClusterLoopSignal::RefreshMetadataResponse(new_metadata) => {
                // TODO: should be called when metadata check is done from different reason then timeout

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
