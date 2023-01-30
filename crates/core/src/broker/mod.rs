use std::{
    collections::{BTreeMap, HashMap},
    io,
    mem::size_of,
    net::SocketAddr,
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
};

use bytes::{Bytes, BytesMut};
use kafka_connector_protocol::{
    api::api_versions::ApiVersionsRequest, api_call::ApiRequest, api_call::ApiResponse,
    api_error::ApiError, api_numbers::ApiNumbers,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::oneshot,
};
use uuid::Uuid;

use self::errors::ConnectionError;

pub mod errors;

#[derive(Debug, Clone)]
pub struct BrokerOptions {
    address: SocketAddr,
    /// should be unique; for log usage
    broker_id: u32,
    client_software_version: String,
    client_version_name: String,
    client_id: String,
}
impl BrokerOptions {
    pub fn new(address: SocketAddr, broker_id: u32) -> Self {
        Self {
            address,
            broker_id,
            client_software_version: env!("CARGO_PKG_VERSION").to_owned(),
            client_version_name: env!("CARGO_PKG_NAME").to_owned(),
            client_id: format!("kafka-connector-{}", Uuid::new_v4()),
        }
    }
}

pub type ActiveRequestList = Arc<Mutex<HashMap<i32, oneshot::Sender<Vec<u8>>>>>;

pub struct Broker {
    options: Arc<BrokerOptions>,
    disconnected_receiver: Option<Receiver<()>>,
    write_stream: Option<OwnedWriteHalf>,
    active_requests: ActiveRequestList,
    supported_api_versions: BTreeMap<u16, (u16, u16)>,
    last_correlation: i32,
    buffer: BytesMut,
}

impl Broker {
    pub fn new(options: BrokerOptions) -> Broker {
        Broker {
            options: Arc::new(options),
            disconnected_receiver: None,
            write_stream: None,
            active_requests: Default::default(),
            last_correlation: 0,
            supported_api_versions: Default::default(),
            buffer: BytesMut::with_capacity(1024), // TODO: Proper size
        }
    }

    pub fn is_connected(&self) -> bool {
        match self.disconnected_receiver.as_ref() {
            Some(receiver) => match receiver.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => false,
                Err(TryRecvError::Empty) => true,
            },
            None => false,
        }
    }

    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        if self.is_connected() {
            return Err(ConnectionError::AlreadyConnected);
        }
        let address = self.options.address;
        log::debug!(
            "Broker-{broker_id}: Connecting to kafka broker {address:?}",
            broker_id = self.options.broker_id
        );

        let connection = TcpStream::connect(address).await?;

        let (connection_read_half, connection_write_half) = connection.into_split();
        let (connection_closed_sender, connection_closed_receiver) = channel::<()>();

        debug_assert!(self.disconnected_receiver.is_none());
        self.disconnected_receiver = Some(connection_closed_receiver);

        debug_assert!(self.write_stream.is_none());
        self.write_stream = Some(connection_write_half);

        debug_assert!(self.active_requests.lock().expect("TODO:").is_empty());
        self.last_correlation = 0;

        tokio::spawn(Broker::listen_loop(
            connection_read_half,
            connection_closed_sender,
            self.active_requests.clone(),
            self.options.clone(),
        ));
        self.get_supported_api_versions().await?;

        log::debug!(
            "Broker-{broker_id}: Connected to kafka broker {address:?}",
            broker_id = self.options.broker_id
        );

        Ok(())
    }

    /// Sends api requests returning the broker response
    /// Should handle retryable errors
    async fn send_api_request_internal<R: ApiRequest>(
        &mut self,
        request: R,
    ) -> Result<R::Response, ApiError> {
        // TODO: retry?
        let request_version = if R::get_api_key() == ApiNumbers::ApiVersions
            && self.supported_api_versions.is_empty()
        {
            R::get_max_supported_version()
        } else {
            let broker_supported_versions = self
                .supported_api_versions
                .get(&(R::get_api_key() as u16))
                .ok_or(ApiError::UnsupportedVersion)?;
            if R::get_max_supported_version() <= broker_supported_versions.1 {
                R::get_max_supported_version()
            } else {
                broker_supported_versions.1
            }
        };

        let correlation = {
            self.last_correlation += 1;
            self.last_correlation
        };

        request.serialize(
            request_version,
            &mut self.buffer,
            correlation,
            &self.options.client_id,
        );
        let req_buf = self.buffer.split();

        let write_stream = self.write_stream.as_mut().expect("TODO:");

        let request_len = req_buf.len() as i32;
        write_stream
            .write_all(&request_len.to_be_bytes())
            .await
            .expect("TODO"); // should not block?
        write_stream.write_all(&req_buf).await.expect("TODO"); // should not block?

        let channel = oneshot::channel();
        {
            let mut guard = self.active_requests.lock().expect("TODO:");
            if (*guard).insert(correlation, channel.0).is_some() {
                panic!("Sending second request with same correlation")
            }
        }

        let response = channel.1.await.expect("TODO:");

        let mut response = Bytes::from(response);
        let (_correlation, response) = if R::get_api_key() == ApiNumbers::ApiVersions {
            let copy = response.clone();
            let ret = R::Response::deserialize(request_version, &mut response);
            if !response.is_empty() {
                response = copy;
                let ret = R::Response::deserialize(0, &mut response);
                debug_assert!(response.is_empty());
                ret
            } else {
                ret
            }
        } else {
            let ret = R::Response::deserialize(request_version, &mut response);
            debug_assert!(response.is_empty());
            ret
        };

        // TODO: Error handling?

        // if let Some(err) = T::get_first_error(&response) {
        //     return Err((KafkaApiCallError::KafkaApiError(err), response));
        // }
        Ok(response)
    }

    async fn get_supported_api_versions(&mut self) -> Result<(), ConnectionError> {
        // Fallback to version 0 for ApiVersions response. If a client sends an ApiVersionsRequest
        // using a version higher than that supported by the broker, a version 0 response is sent
        // to the client indicating UNSUPPORTED_VERSION. When the client receives the response, it
        // falls back while parsing it into a Struct which means that the version received by this
        // method is not necessary the real one. It may be version 0 as well.

        self.supported_api_versions.clear();
        let mut request = ApiVersionsRequest::<3>::default();
        // TODO: mutable builder
        request.with_client_software_name(self.options.client_version_name.clone());
        request.with_client_software_version(self.options.client_software_version.clone());

        let resp = self.send_api_request_internal(request).await;

        match resp {
            Ok(response) => {
                self.supported_api_versions = response
                    .api_keys()
                    .iter()
                    .map(|x| {
                        (
                            x.api_key() as u16,
                            (x.min_version() as u16, x.max_version() as u16),
                        )
                    })
                    .collect();
            }
            Err(ApiError::UnsupportedVersion) => {
                let request = ApiVersionsRequest::<0>::default();
                let resp = self.send_api_request_internal(request).await;
                match resp {
                    Ok(response) => {
                        self.supported_api_versions = response
                            .api_keys()
                            .iter()
                            .map(|x| {
                                (
                                    x.api_key() as u16,
                                    (x.min_version() as u16, x.max_version() as u16),
                                )
                            })
                            .collect();
                    }
                    Err(ApiError::UnsupportedVersion) => {
                        return Err(ConnectionError::UnsupportedVersion);
                    }
                    Err(err) => Err(err)?,
                }
            }
            Err(err) => Err(err)?,
        }

        Ok(())
    }

    async fn listen_loop(
        mut read_half: OwnedReadHalf,
        connection_closed_sender: Sender<()>,
        active_requests: ActiveRequestList,
        options: Arc<BrokerOptions>,
    ) {
        let broker_id = options.broker_id;
        loop {
            let mut response_buf = [0; 4];
            match read_half.read_exact(&mut response_buf).await {
                Ok(_) => (),
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                    let _ = connection_closed_sender.send(());
                    log::debug!("Broker-{broker_id}: Disconnected",);
                    break;
                }
                Err(err) => {
                    log::error!(
                        "Broker-{broker_id}: Unknown error while communicating with kafka {err:?}"
                    );
                    let _ = connection_closed_sender.send(());
                    break;
                }
            }
            let response_length = i32::from_be_bytes(response_buf);
            log::trace!("Broker-{broker_id}: Received message length {response_length}");

            let mut message_buf = vec![0; response_length as usize];
            match read_half.read_exact(&mut message_buf).await {
                Ok(_) => (),
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                    let _ = connection_closed_sender.send(());
                    break;
                }
                Err(err) => {
                    log::error!(
                        "Broker-{broker_id}: Unknown error while communicating with kafka {err:?}"
                    );
                    let _ = connection_closed_sender.send(());
                    break;
                }
            }

            let correlation_id = i32::from_be_bytes(
                message_buf
                    .split_at(size_of::<i32>())
                    .0
                    .try_into()
                    .expect("Failed to read correlation_id"),
            );

            log::trace!(
                "Broker-{broker_id}: Received message: {:#02x}",
                Bytes::from(message_buf.clone())
            );

            if active_requests
                .lock()
                .expect("Encountered mutex poisoning, killing broker connection")
                .remove(&correlation_id)
                .expect("Unknown request for received response")
                .send(message_buf)
                .is_err()
            {
                log::debug!("Broker-{broker_id}: Dropping response, no active listener");
            }
        }
        log::trace!("Broker-{broker_id}: Listen loop exiting");
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils::BROKER_1;
    use crate::test_utils::BROKER_2;
    use crate::test_utils::BROKER_3;

    use super::*;

    fn init() {
        let _ = env_logger::builder().try_init();
    }

    /// Check if client can connect to kafka correctly
    ///
    /// Additionally checks if all three brokers used in other tests are up and running
    #[tokio::test]
    async fn connects_to_kafka_brokers() {
        init();

        let options = BrokerOptions::new(BROKER_1.parse().unwrap(), 1);
        let mut broker_1 = Broker::new(options.clone());
        broker_1.connect().await.unwrap();
        drop(broker_1);

        let options = BrokerOptions::new(BROKER_2.parse().unwrap(), 2);
        let mut broker_2 = Broker::new(options.clone());
        broker_2.connect().await.unwrap();

        let options = BrokerOptions::new(BROKER_3.parse().unwrap(), 3);
        let mut broker_3 = Broker::new(options);
        broker_3.connect().await.unwrap();
    }

    // TODO: test errors
}
