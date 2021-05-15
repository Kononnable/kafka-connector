pub mod error;
pub mod options;

use std::{
    cmp::{max, min},
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
    u8,
};

use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{mpsc::UnboundedSender, RwLock},
};
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::oneshot,
};

use kafka_connector_protocol::{
    api::{
        api_versions::ApiVersionsRequest,
        metadata::{MetadataRequest, MetadataRequestTopics0},
        ApiNumbers,
    },
    api_error::ApiError,
    custom_types::tag_buffer::TagBuffer,
    ApiCall,
};

use crate::{
    cluster::{
        cluster_loop::ClusterLoopSignal,
        metadata::{Metadata, PartitionMetadata, TopicMetadata},
    },
    utils::is_api_error_retriable,
};

use self::{error::KafkaApiCallError, options::KafkaClientOptions};

#[derive(Debug)]
pub struct Connection {
    active_requests: Arc<std::sync::Mutex<HashMap<i32, oneshot::Sender<Vec<u8>>>>>,
    socket_writer: OwnedWriteHalf,
    connection_closed_receiver: oneshot::Receiver<()>,
    last_correlation: i32,
}

#[derive(Debug)]
pub struct Broker {
    addr: SocketAddr,
    supported_versions: HashMap<u16, (u16, u16)>,
    send_buffer: BytesMut,
    options: Arc<KafkaClientOptions>,
    connection: Connection,
}

impl Broker {
    pub async fn new_wait(
        addr: SocketAddr,
        options: Arc<KafkaClientOptions>,
    ) -> Result<Broker, KafkaApiCallError> {
        let connection = TcpStream::connect(addr).await?;
        let (read_half, write_half) = connection.into_split();
        let (connection_closed_sender, connection_closed_receiver) = oneshot::channel::<()>();
        let active_requests = Arc::new(Mutex::new(HashMap::new()));
        tokio::spawn(Broker::listen_loop(
            read_half,
            connection_closed_sender,
            active_requests.clone(),
        ));
        let connection = Connection {
            active_requests,
            connection_closed_receiver,
            last_correlation: 0,
            socket_writer: write_half,
        };

        let mut broker = Broker {
            addr,
            supported_versions: HashMap::new(),
            options,
            send_buffer: BytesMut::with_capacity(4096), // TODO: Change size(?), change with capacity so it can grow
            connection,
        };
        broker.get_supported_api_versions().await?;
        Ok(broker)
    }
    pub(crate) fn new_no_wait(
        addr: SocketAddr,
        options: Arc<KafkaClientOptions>,
        sender: UnboundedSender<ClusterLoopSignal>,
        broker_id: i32,
    ) {
        // TODO: What if future is dropped while connecting/after connection - check if handled correctly
        tokio::spawn(async move {
            let result = Broker::new_wait(addr, options).await;
            match result {
                Ok(broker) => {
                    sender
                        .send(ClusterLoopSignal::BrokerConnected(broker_id, broker))
                        .unwrap(); // Cluster loop dropped before broker drop
                }
                Err(_e) => {
                    sender
                        .send(ClusterLoopSignal::BrokerDisconnected(broker_id))
                        .unwrap(); // Cluster loop dropped before broker drop
                }
            }
        });
    }

    async fn listen_loop(
        mut read_half: OwnedReadHalf,
        connection_closed_sender: oneshot::Sender<()>,
        active_requests: Arc<std::sync::Mutex<HashMap<i32, oneshot::Sender<Vec<u8>>>>>,
    ) {
        log::debug!("Broker listen_loop start");
        // TODO: Remove unwraps
        loop {
            let mut size: [u8; 4] = [0, 0, 0, 0];
            if let Err(err) = read_half.read_exact(&mut size).await {
                match err.kind() {
                    std::io::ErrorKind::UnexpectedEof => {
                        log::debug!("Broker connection closed");
                        break;
                    }
                    _ => {
                        log::error!(
                            "Unknown error while communicating with kafka broker {:?}",
                            err
                        );
                        break;
                    }
                }
            }
            let cap = i32::from_be_bytes(size);
            let mut buf2 = vec![0; cap as usize];
            if let Err(err) = read_half.read_exact(&mut buf2).await {
                log::error!(
                    "Unknown error while communicating with kafka broker {:?}",
                    err
                );
                break;
            }
            log::trace!("Received bytes: {:?}", buf2);

            let correlation_id = i32::from_be_bytes([buf2[0], buf2[1], buf2[2], buf2[3]]);
            log::trace!("Correlation id: {}", correlation_id);
            {
                let mut guard = active_requests.lock().unwrap();
                let entry = (*guard).remove(&correlation_id);
                let channel = entry.unwrap();

                channel.send(buf2).unwrap();
            }
        }
        if connection_closed_sender.send(()).is_err() {
            // No need to do anything broker.connection is already dropped
        };

        // TODO: make it as blocked somehow(so no new requests are inserted)?
        let mut guard = active_requests.lock().unwrap();
        guard.clear();

        log::debug!("Broker dropped");
    }

    pub(crate) async fn refresh_cluster_metadata(
        &mut self,
        metadata: Arc<RwLock<Metadata>>,
        sender: UnboundedSender<ClusterLoopSignal>,
    ) {
        let topics = {
            let metadata = metadata.read().await;
            metadata
                .topics
                .keys()
                .map(|x| MetadataRequestTopics0 {
                    name: x.clone(),
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
        let response = self.run_api_call_with_retry(request, None).await.unwrap(); // TODO: remove unwrap
        let brokers = response
            .brokers
            .into_iter()
            .map(|x| (x.node_id, (x.host, x.port)))
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
        sender
            .send(ClusterLoopSignal::RefreshMetadataResponse(metadata))
            .unwrap(); // Cluster loop dropped before broker drop
    }
    pub async fn run_api_call<T>(
        &mut self,
        request: &T,
        api_version: Option<u16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall,
    {
        let api_version = if T::get_api_key() == ApiNumbers::ApiVersions {
            api_version.unwrap_or_default()
        } else {
            let supported_versions = self
                .supported_versions
                .get(&(T::get_api_key() as u16))
                .ok_or_else(|| KafkaApiCallError::UnsupportedKafkaApiVersion {
                    api: T::get_api_key(),
                    version: 0,
                })?;
            match api_version {
                Some(v) => {
                    if v >= supported_versions.0
                        && v <= supported_versions.1
                        && v >= T::get_min_supported_version()
                        && v <= T::get_max_supported_version()
                    {
                        v
                    } else {
                        return Err(KafkaApiCallError::UnsupportedKafkaApiVersion {
                            api: T::get_api_key(),
                            version: v,
                        });
                    }
                }
                None => {
                    let min_supported = max(supported_versions.0, T::get_min_supported_version());
                    let max_supported = min(supported_versions.1, T::get_max_supported_version());
                    if max_supported < min_supported {
                        return Err(KafkaApiCallError::UnsupportedKafkaApiVersion {
                            api: T::get_api_key(),
                            version: T::get_min_supported_version(),
                        });
                    }
                    max_supported
                }
            }
        };
        let correlation = {
            self.connection.last_correlation += 1;
            self.connection.last_correlation
        };
        request.serialize(
            api_version,
            &mut self.send_buffer,
            correlation,
            &self.options.client_id,
        );

        let mut buf2 = self.send_request(correlation).await;
        let (_correlation, response) = T::deserialize_response(api_version, &mut buf2);
        if let Some(err) = T::get_first_error(&response) {
            return Err(KafkaApiCallError::KafkaApiError(err));
        }
        Ok(response)
    }

    async fn send_request(&mut self, correlation: i32) -> Bytes {
        let len = self.send_buffer.len() as i32;
        // TODO: safely remove unwrap
        self.connection
            .socket_writer
            .write_all(&len.to_be_bytes())
            .await
            .unwrap();
        self.connection
            .socket_writer
            .write_all(&self.send_buffer)
            .await
            .unwrap();
        self.send_buffer.clear();
        let channel = oneshot::channel();
        {
            let mut guard = self.connection.active_requests.lock().unwrap();
            if (*guard).insert(correlation, channel.0).is_some() {
                panic!("Sending second request with same correlation")
            }
        }
        let response = channel.1.await.unwrap();
        Bytes::from(response)
    }

    async fn get_supported_api_versions(&mut self) -> Result<(), KafkaApiCallError> {
        let response = self
            .run_api_call(&ApiVersionsRequest::default(), Some(0))
            .await?;
        if response.error_code != 0 {
            return Err(KafkaApiCallError::KafkaApiError(ApiError::from(
                response.error_code,
            )));
        }
        self.supported_versions.clear();
        for api_key in response.api_keys {
            self.supported_versions.insert(
                api_key.api_key as u16,
                (api_key.min_version as u16, api_key.max_version as u16),
            );
        }
        Ok(())
    }

    /// Run api call with automatic retry on errors on which message can just be resend
    pub async fn run_api_call_with_retry<T>(
        &mut self,
        request: T,
        api_version: Option<u16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall,
    {
        let mut response = self.run_api_call(&request, api_version).await;
        for _i in 0..=3 {
            // TODO: Extract to config value
            if let Err(KafkaApiCallError::KafkaApiError(api_error)) = response {
                if is_api_error_retriable(api_error) {
                    tokio::time::sleep(Duration::from_millis(100)).await; // TODO: Extract to config value, gradually increase wait duration, logging
                    response = self.run_api_call(&request, api_version).await;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return response;
    }
}

#[cfg(test)]
mod tests {
    use std::net::ToSocketAddrs;

    use super::*;
    use anyhow::Result;

    const BROKER: &str = "127.0.0.1:9092";
    const CLIENT_ID: &str = "kafka-connector-test";

    #[tokio::test]
    async fn should_fetch_api_versions_after_connect() -> Result<()> {
        let broker_client = Broker::new_wait(
            BROKER.to_socket_addrs()?.next().unwrap(),
            Arc::new(get_test_client_options()),
        )
        .await?;

        assert!(!broker_client.supported_versions.is_empty());
        Ok(())
    }

    fn get_test_client_options() -> KafkaClientOptions {
        KafkaClientOptions::builder()
            .client_id(CLIENT_ID.to_owned())
            .build()
    }
}
