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
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::oneshot,
};

use kafka_connector_protocol::{
    api::{api_versions::ApiVersionsRequest, ApiNumbers},
    api_error::ApiError,
    ApiCall,
};

use crate::utils::is_api_error_retriable;

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
    supported_versions: HashMap<u16, (u16, u16)>, // TODO: Change later(?)
    send_buffer: BytesMut,
    options: Arc<KafkaClientOptions>,
    connection: Option<Connection>,
}

impl Broker {
    pub fn new(addr: SocketAddr, options: Arc<KafkaClientOptions>) -> Broker {
        Broker {
            addr,
            supported_versions: HashMap::new(),
            options,
            send_buffer: BytesMut::with_capacity(4096), // TODO: Change size(?), change with capacity so it can grow
            connection: None,
        }
    }

    pub fn is_connected(&mut self) -> bool {
        if let Some(connection) = &mut self.connection {
            if connection.connection_closed_receiver.try_recv().is_ok() {
                self.connection = None;
                return false;
            }
            return true;
        }
        false
    }

    pub fn disconnect(&mut self) {
        // TODO: is custom logic necessary? e.g. letting know group controller for consumer
        if !self.is_connected() {
            return;
        }
        self.connection.take();
    }

    pub async fn connect(&mut self) -> Result<(), KafkaApiCallError> {
        let connection = TcpStream::connect(self.addr).await?;
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
        self.connection = Some(connection);

        self.get_supported_api_versions().await?;

        Ok(())
    }

    async fn listen_loop(
        mut read_half: OwnedReadHalf,
        connection_closed_sender: oneshot::Sender<()>,
        active_requests: Arc<std::sync::Mutex<HashMap<i32, oneshot::Sender<Vec<u8>>>>>,
    ) {
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
        if !self.is_connected() {
            return Err(KafkaApiCallError::BrokerNotConnected());
        }
        let correlation = {
            let connection = self.connection.as_mut().unwrap();
            connection.last_correlation += 1;
            connection.last_correlation
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
        let connection = self.connection.as_mut().unwrap();
        connection
            .socket_writer
            .write_all(&len.to_be_bytes())
            .await
            .unwrap();
        connection
            .socket_writer
            .write_all(&self.send_buffer)
            .await
            .unwrap();
        self.send_buffer.clear();
        let channel = oneshot::channel();
        {
            let mut guard = connection.active_requests.lock().unwrap();
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

    // TODO:
    #[tokio::test]
    async fn should_fetch_api_versions_after_connect() -> Result<()> {
        let mut broker_client = Broker::new(
            BROKER.to_socket_addrs()?.next().unwrap(),
            Arc::new(get_test_client_options()),
        );
        broker_client.connect().await?;

        assert!(!broker_client.supported_versions.is_empty());
        Ok(())
    }

    fn get_test_client_options() -> KafkaClientOptions {
        KafkaClientOptions::builder()
            .client_id(CLIENT_ID.to_owned())
            .build()
    }
}
