pub(crate) mod cluster_loop;
pub mod error;
pub mod metadata;

use std::{net::ToSocketAddrs, sync::Arc};

use futures_util::future::select_all;
use kafka_connector_protocol::api::metadata::{MetadataRequest, MetadataResponse0};
use log::{debug, warn};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    RwLock,
};

use crate::broker::{options::KafkaClientOptions, Broker};

use self::{
    cluster_loop::ClusterLoopSignal, error::ClusterClientCreationError, metadata::Metadata,
};

#[derive(Debug)]
pub struct Cluster {
    metadata: Arc<RwLock<Metadata>>,
    options: Arc<KafkaClientOptions>,
    loop_signal_sender: UnboundedSender<ClusterLoopSignal>,
}

impl Cluster {
    pub async fn new(
        broker_addrs: impl ToSocketAddrs,
        options: KafkaClientOptions,
    ) -> Result<Arc<Cluster>, ClusterClientCreationError> {
        let addresses: Vec<_> = broker_addrs
            .to_socket_addrs()
            .map_err(ClusterClientCreationError::AddressRecognitionFailed)?
            .collect();
        if addresses.is_empty() {
            return Err(ClusterClientCreationError::NoClusterAddressFound {});
        }
        let options = Arc::new(options);

        let futures = addresses.into_iter().map(|addr| {
            let options = options.clone();
            Box::pin(async move {
                let mut client = Broker::new_wait(addr, options.clone()).await?;
                let metadata_request = MetadataRequest {
                    ..Default::default()
                };
                let metadata = client
                    .run_api_call_with_retry(metadata_request, None)
                    .await?;
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
        let cluster = Arc::new(Cluster {
            metadata: Default::default(),
            options,
            loop_signal_sender,
        });

        tokio::spawn(cluster_loop::cluster_loop(
            clients,
            loop_signal_receiver,
            cluster.clone(),
        ));

        Ok(cluster)
    }
}

impl Drop for Cluster {
    fn drop(&mut self) {
        debug!("Cluster is being dropped, closing all kafka connections");
        self.loop_signal_sender
            .send(ClusterLoopSignal::Shutdown)
            .expect("Cluster loop should be alive.")
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
}
