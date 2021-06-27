pub mod error;
pub mod options;

use std::{
    cmp::{max, min},
    collections::HashMap,
    convert::TryInto,
    mem::size_of,
    net::SocketAddr,
    sync::{Arc, Mutex},
    u8,
};

use bytes::{Bytes, BytesMut};
use futures::TryFutureExt;
use log::trace;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::UnboundedSender,
};
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::oneshot,
};

use kafka_connector_protocol::{
    api::{api_versions::ApiVersionsRequest, ApiNumbers},
    ApiCall,
};

use crate::{cluster::cluster_loop::ClusterLoopSignal, utils::is_api_error_retriable};

use self::{error::KafkaApiCallError, options::KafkaClientOptions};

pub type ActiveRequestList = Arc<std::sync::Mutex<Option<HashMap<i32, oneshot::Sender<Vec<u8>>>>>>;

#[derive(Debug)]
pub struct Broker<S: BrokerState> {
    pub(crate) addr: SocketAddr,
    pub(crate) options: Arc<KafkaClientOptions>,
    send_buffer: BytesMut,
    state: S,
}

#[derive(Debug)]
pub struct Initializing {}

#[derive(Debug)]
pub struct Alive {
    supported_versions: HashMap<u16, (u16, u16)>,
    active_requests: ActiveRequestList,
    socket_writer: OwnedWriteHalf,
    connection_closed_receiver: oneshot::Receiver<()>,
    last_correlation: i32,
}

pub trait BrokerState {}
impl BrokerState for Initializing {}
impl BrokerState for Alive {}

impl Broker<Initializing> {
    pub fn new(addr: SocketAddr, options: Arc<KafkaClientOptions>) -> Broker<Initializing> {
        let send_buffer = BytesMut::with_capacity(options.buffer_size);

        Broker {
            addr,
            state: Initializing {},
            options,
            send_buffer,
        }
    }

    pub async fn new_wait(self) -> Result<Broker<Alive>, KafkaApiCallError> {
        let connection = TcpStream::connect(self.addr).await?;
        let (read_half, write_half) = connection.into_split();
        let (connection_closed_sender, connection_closed_receiver) = oneshot::channel::<()>();
        let active_requests = Arc::new(Mutex::new(Some(HashMap::new())));
        tokio::spawn(Broker::<Alive>::listen_loop(
            read_half,
            connection_closed_sender,
            active_requests.clone(),
        ));

        let mut broker = Broker {
            addr: self.addr,
            options: self.options,
            send_buffer: self.send_buffer,
            state: Alive {
                active_requests,
                connection_closed_receiver,
                last_correlation: 0,
                socket_writer: write_half,
                supported_versions: Default::default(),
            },
        };
        broker.get_supported_api_versions().await?;
        Ok(broker)
    }

    pub(crate) fn new_no_wait(&self, sender: UnboundedSender<ClusterLoopSignal>, broker_id: i32) {
        let broker = Broker::new(self.addr, self.options.clone());
        // TODO: What if future is dropped while connecting/after connection - check if handled correctly
        tokio::spawn(async move {
            let result = broker.new_wait().await;
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
}

impl Broker<Alive> {
    pub async fn run_api_call<T>(
        &mut self,
        request: &T,
        api_version: Option<u16>,
    ) -> Result<T::Response, (KafkaApiCallError, T::Response)>
    where
        T: ApiCall,
    {
        let api_version = if T::get_api_key() == ApiNumbers::ApiVersions {
            api_version.unwrap_or_default()
        } else {
            let supported_versions = self
                .state
                .supported_versions
                .get(&(T::get_api_key() as u16))
                .ok_or_else(|| {
                    (
                        KafkaApiCallError::UnsupportedKafkaApiVersion {
                            api: T::get_api_key(),
                            version: 0,
                        },
                        T::Response::default(),
                    )
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
                        return Err((
                            KafkaApiCallError::UnsupportedKafkaApiVersion {
                                api: T::get_api_key(),
                                version: v,
                            },
                            T::Response::default(),
                        ));
                    }
                }
                None => {
                    let min_supported = max(supported_versions.0, T::get_min_supported_version());
                    let max_supported = min(supported_versions.1, T::get_max_supported_version());
                    if max_supported < min_supported {
                        return Err((
                            KafkaApiCallError::UnsupportedKafkaApiVersion {
                                api: T::get_api_key(),
                                version: T::get_min_supported_version(),
                            },
                            T::Response::default(),
                        ));
                    }
                    max_supported
                }
            }
        };
        let correlation = {
            self.state.last_correlation += 1;
            self.state.last_correlation
        };
        request.serialize(
            api_version,
            &mut self.send_buffer,
            correlation,
            &self.options.client_id,
        );

        let mut response_buffer = self.send_request(correlation).await;
        let (_correlation, response) = T::deserialize_response(api_version, &mut response_buffer);
        if let Some(err) = T::get_first_error(&response) {
            return Err((KafkaApiCallError::KafkaApiError(err), response));
        }
        Ok(response)
    }

    async fn send_request(&mut self, correlation: i32) -> Bytes {
        let len = self.send_buffer.len() as i32;
        // TODO: safely remove unwrap
        self.state
            .socket_writer
            .write_all(&len.to_be_bytes())
            .await
            .unwrap();
        self.state
            .socket_writer
            .write_all(&self.send_buffer)
            .await
            .unwrap();
        self.send_buffer.clear();
        let channel = oneshot::channel();
        {
            let mut guard = self.state.active_requests.lock().unwrap();
            if let Some(active_requests) = guard.as_mut() {
                if active_requests.insert(correlation, channel.0).is_some() {
                    panic!("Sending second request with same correlation")
                }
            }
        }
        let response = channel.1.await.unwrap();
        Bytes::from(response)
    }

    async fn get_supported_api_versions(&mut self) -> Result<(), KafkaApiCallError> {
        let response = self
            .run_api_call_with_retry(ApiVersionsRequest::default(), Some(0))
            .await?;

        self.state.supported_versions.clear();
        for api_key in response.api_keys {
            self.state.supported_versions.insert(
                api_key.api_key as u16,
                (api_key.min_version as u16, api_key.max_version as u16),
            );
        }
        Ok(())
    }

    // TODO: Change somehow
    pub async fn run_api_call_with_retry_raw<T>(
        &mut self,
        request: T,
        api_version: Option<u16>,
    ) -> Result<T::Response, (KafkaApiCallError, T::Response)>
    where
        T: ApiCall,
    {
        let mut response = self.run_api_call(&request, api_version).await;
        for _i in 0..=self.options.retries {
            if let Err((KafkaApiCallError::KafkaApiError(api_error), _)) = response {
                if is_api_error_retriable(api_error) {
                    trace!(
                        "Api call failed, retrying in {:?}",
                        self.options.retry_backoff_ms
                    );
                    tokio::time::sleep(self.options.retry_backoff_ms).await;
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
    /// Run api call with automatic retry on errors on which message can just be resend
    pub async fn run_api_call_with_retry<T>(
        &mut self,
        request: T,
        api_version: Option<u16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall,
    {
        self.run_api_call_with_retry_raw(request, api_version)
            .map_err(|x| x.0)
            .await
    }
}

impl<S> Broker<S>
where
    S: BrokerState,
{
    async fn listen_loop(
        mut read_half: OwnedReadHalf,
        connection_closed_sender: oneshot::Sender<()>,
        active_requests: ActiveRequestList,
    ) {
        log::debug!("Broker listen_loop start");
        loop {
            let mut buffer_size = [0; 4];
            if let Err(err) = read_half.read_exact(&mut buffer_size).await {
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
            let message_size = i32::from_be_bytes(buffer_size);
            let mut message_buffer = vec![0; message_size as usize];
            if let Err(err) = read_half.read_exact(&mut message_buffer).await {
                log::error!(
                    "Unknown error while communicating with kafka broker {:?}",
                    err
                );
                break;
            }
            log::trace!("Received message");

            let correlation_id = i32::from_be_bytes(
                message_buffer
                    .split_at(size_of::<i32>())
                    .0
                    .try_into()
                    .expect("Failed to read correlation_id"),
            );
            log::trace!("Correlation id: {}", correlation_id);
            if let Ok(mut guard) = active_requests.lock() {
                let entry = guard
                    .as_mut()
                    .expect("Broker is in closed state")
                    .remove(&correlation_id);
                let channel = entry.expect("Unknown request for received response");
                if channel.send(message_buffer).is_err() {
                    log::debug!("Dropping response, no active listener");
                }
            } else {
                log::error!("Encountered mutex poisoning, killing broker connection");
                break;
            }
        }
        if connection_closed_sender.send(()).is_err() {
            // No need to do anything broker.connection is already dropped
        };

        if let Ok(mut guard) = active_requests.lock() {
            *guard = None;
        } else {
            log::error!("Encountered mutex poisoning, killing broker connection");
        }

        log::debug!("Broker listen_loop end");
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
        let broker_client = Broker::new(
            BROKER.to_socket_addrs()?.next().unwrap(),
            Arc::new(get_test_client_options()),
        )
        .new_wait()
        .await?;

        assert!(!broker_client.state.supported_versions.is_empty());
        Ok(())
    }

    fn get_test_client_options() -> KafkaClientOptions {
        KafkaClientOptions::builder()
            .client_id(CLIENT_ID.to_owned())
            .build()
    }
}
