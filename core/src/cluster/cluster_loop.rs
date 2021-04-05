use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::broker::Broker;
use log::{debug, trace};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::StreamExt;

use super::Cluster;

#[derive(Debug)]
pub(super) enum ClusterLoopSignal {
    /// Disconnect from kafka brokers, clean up
    Shutdown,
    /// Timed event, every x seconds we should check for metadata changes
    PerformMetadataUpdate,
    /// Broker response for metadata update request - e.g. because broker received an error of outdated metadata
    MetadataUpdate(()), // BrokerConnected,
                        // BrokerDisconnected
}

pub(super) async fn cluster_loop(
    clients: HashMap<i32, Broker>,
    mut loop_signal_receiver: UnboundedReceiver<ClusterLoopSignal>,
    cluster: Arc<Cluster>,
) {
    // TODO: start client loops(connect, fetch metadata etc.)
    // handle broker connections + disconnected

    let metadata_refresh_stream = futures::stream::repeat(()).throttle(Duration::from_secs(2)); // TODO: configurable?,  change value

    while let Some(signal) = loop_signal_receiver.recv().await {
        match signal {
            ClusterLoopSignal::Shutdown => {
                break;
            }
            ClusterLoopSignal::PerformMetadataUpdate => {
                todo!()
            }
            ClusterLoopSignal::MetadataUpdate(_) => {
                todo!()
            }
        }
    }
    // TODO: close connections
    trace!("Cluster loop close")
}
