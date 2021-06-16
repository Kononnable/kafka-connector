pub(crate) mod cluster_loop;
pub mod error;
pub mod metadata;

use std::{collections::HashMap, net::ToSocketAddrs, sync::Arc};

use futures_util::future::select_all;
use kafka_connector_protocol::{
    api::metadata::{MetadataRequest, MetadataResponse0},
    ApiCall,
};
use log::{debug, warn};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    RwLock,
};

use crate::broker::{error::KafkaApiCallError, options::KafkaClientOptions, Broker};

use self::{
    cluster_loop::{BrokerState, ClusterLoopSignal},
    error::ClusterClientCreationError,
    metadata::Metadata,
};

#[derive(Debug)]
pub(crate) struct ClusterInner {
    pub brokers: RwLock<HashMap<i32, BrokerState>>,
    pub metadata: Arc<RwLock<Metadata>>,
    pub options: Arc<KafkaClientOptions>,
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
            let options = options.clone();
            Box::pin(async move {
                let broker = Broker::new(addr, options.clone());
                let mut client = broker.new_wait().await?;
                let metadata_request = MetadataRequest::default();
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
        let cluster = Cluster {
            inner: Arc::new(ClusterInner {
                metadata: Default::default(),
                options,
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
    pub async fn send_request_to_any_broker<T>(
        &self,
        request: T,
        api_version: Option<u16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall,
    {
        // TODO: Rename
        // TODO: remove unwraps
        // TODO: What if no broker connected yet
        let mut brokers = self.inner.brokers.write().await;
        if let BrokerState::Alive(broker) = brokers
            .iter_mut()
            .find(|broker| matches!(broker.1, BrokerState::Alive { .. }))
            .unwrap()
            .1
        {
            broker.run_api_call_with_retry(request, api_version).await
        } else {
            todo!()
        }
    }
}

impl Drop for Cluster {
    fn drop(&mut self) {
        debug!("Cluster is being dropped, closing all kafka connections");
        self.inner
            .loop_signal_sender
            .send(ClusterLoopSignal::Shutdown)
            .expect("Cluster loop should be alive.")
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        consumer::{self, options::ConsumerOptions},
        producer::{self, options::ProducerOptions, record::ProducerRecord},
    };

    use super::*;
    use anyhow::Result;
    use log::trace;
    use tokio::{pin, time::sleep};
    use tokio_stream::StreamExt;

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

    #[tokio::test]
    async fn should_produce_and_consume_single_message() -> Result<()> {
        // TODO: Change to a proper tests
        let cluster = Arc::new(Cluster::new(BROKER, get_test_client_options()).await?);

        sleep(Duration::from_secs(2)).await;

        let mut producer = producer::Producer::new(cluster.clone(), ProducerOptions {}).await;
        let consumer = consumer::Consumer::new(
            cluster,
            ConsumerOptions {
                group_id: "kafka-connector-test".to_owned(),
                topics: vec!["kafka-connector-test".to_owned()],
            },
        )
        .await;

        sleep(Duration::from_secs(2)).await;

        producer
            .send(
                ProducerRecord::builder()
                    .topic("kafka-connector-test".to_owned())
                    .payload(b"kafka-connector-test".to_vec())
                    .build(),
            )
            .await;

        sleep(Duration::from_secs(3)).await;

        let stream = consumer.stream().await.unwrap();
        pin!(stream);

        stream.try_next().await.unwrap();

        Ok(())
    }
}
