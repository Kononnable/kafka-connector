pub mod error;
pub mod metadata;

use std::{collections::HashMap, net::ToSocketAddrs, sync::Arc};

use futures_util::future::select_all;
use kafka_connector_protocol::api::metadata::{MetadataRequest, MetadataResponse0};
use log::warn;

use crate::broker::{options::KafkaClientOptions, Broker};

use self::{error::ClusterClientCreationError, metadata::Metadata};

#[derive(Debug)]
pub struct Cluster {
    clients: HashMap<i32, Broker>,
    metadata: Metadata,
    options: Arc<KafkaClientOptions>,
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
        let mut init_clients = addresses
            .into_iter()
            .map(|addr| Broker::new(addr, options.clone()))
            .collect::<Vec<_>>();

        let futures = init_clients.iter_mut().map(|client| {
            Box::pin(async move {
                client.connect().await?;
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
        let clients2 = metadata
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
                Some((Broker::new(addr, options.clone()), broker.node_id))
            })
            .collect::<Vec<_>>();

        if clients2.is_empty() {
            return Err(ClusterClientCreationError::NoClusterAddressFound());
        }

        let mut clients = HashMap::new();
        for client in clients2 {
            clients.insert(client.1, client.0);
        }
        // TODO: start client loops(connect, fetch metadata etc.)

        Ok(Cluster {
            clients,
            metadata: Metadata::default(),
            options,
        })
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
