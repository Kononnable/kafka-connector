use crate::broker::connection::{BrokerConnection, BrokerConnectionInitializationError};
use crate::broker::controller::ResponseChannel;
use crate::cluster::error::ApiCallError;
use crate::{
    broker::controller::{ApiRequestMessage, BrokerControllerStatus},
    cluster::options::ClusterControllerOptions,
};
use bytes::BytesMut;
use indexmap::IndexMap;
use kafka_connector_protocol::ApiRequest;
use kafka_connector_protocol::api_versions_request::ApiVersionsRequest;
use kafka_connector_protocol::api_versions_response::{
    ApiVersionsResponse, ApiVersionsResponseKey,
};
use kafka_connector_protocol::request_header::RequestHeader;
use kafka_connector_protocol::response_header::ResponseHeader;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task::{JoinError, JoinHandle};
use tokio::time::{Instant, Sleep, sleep_until, timeout};
use tracing::{debug, instrument};

struct QueuedApiCall {
    pub timeout: Instant,
    pub request: ApiRequestMessage,
}

struct BrokerLoopStatus {
    inner: BrokerLoopStatusInner,
    status: Arc<RwLock<BrokerControllerStatus>>,
}
impl AsRef<BrokerLoopStatusInner> for BrokerLoopStatus {
    fn as_ref(&self) -> &BrokerLoopStatusInner {
        &self.inner
    }
}
impl AsMut<BrokerLoopStatusInner> for BrokerLoopStatus {
    fn as_mut(&mut self) -> &mut BrokerLoopStatusInner {
        // TODO: Using writing to dereferenced as_mut may invalidate internal state (synchronization with status field)
        &mut self.inner
    }
}
impl BrokerLoopStatus {
    pub fn new(status: Arc<RwLock<BrokerControllerStatus>>) -> BrokerLoopStatus {
        let mut status = BrokerLoopStatus {
            inner: BrokerLoopStatusInner::Disconnected {
                backoff_timeout: Instant::now(),
            },
            status,
        };
        // Make sure internal state is in sync
        status.set(BrokerLoopStatusInner::Disconnected {
            backoff_timeout: Instant::now(),
        });
        status
    }
    pub fn set(&mut self, new_inner: BrokerLoopStatusInner) {
        let status = match &new_inner {
            BrokerLoopStatusInner::Disconnected { .. } => BrokerControllerStatus::Disconnected,
            BrokerLoopStatusInner::Connecting { .. } => BrokerControllerStatus::Connecting,
            BrokerLoopStatusInner::Connected { .. } => BrokerControllerStatus::Connected,
        };

        self.inner = new_inner;
        let mut guard = self.status.write().unwrap();
        *guard = status;
    }
}

enum BrokerLoopStatusInner {
    Disconnected {
        backoff_timeout: Instant,
    },
    Connecting {
        connection_establish_task: JoinHandle<
            Result<(BrokerConnection, ApiVersionsResponse), BrokerConnectionInitializationError>,
        >,
    },
    Connected {
        connection: BrokerConnection,
    },
}

pub struct BrokerLoop {
    _node_id: i32,
    address: String,
    options: Arc<ClusterControllerOptions>,
    api_request_receiver: mpsc::UnboundedReceiver<ApiRequestMessage>,
    api_call_queue: VecDeque<QueuedApiCall>,
    api_calls_in_transit: HashMap<i32, (ResponseChannel, Instant)>,
    supported_api_versions: Arc<RwLock<IndexMap<i16, ApiVersionsResponseKey>>>,
    loop_status: BrokerLoopStatus,
}

impl BrokerLoop {
    #[instrument(level = "debug", skip(api_request_receiver))]
    pub async fn start(
        address: String,
        api_request_receiver: mpsc::UnboundedReceiver<ApiRequestMessage>,
        options: Arc<ClusterControllerOptions>,
        node_id: i32,
        supported_api_versions: Arc<RwLock<IndexMap<i16, ApiVersionsResponseKey>>>,
        status: Arc<RwLock<BrokerControllerStatus>>,
    ) {
        BrokerLoop {
            address,
            api_request_receiver,
            options,
            _node_id: node_id,
            api_call_queue: VecDeque::new(),
            loop_status: BrokerLoopStatus::new(status),
            api_calls_in_transit: HashMap::new(),
            supported_api_versions,
        }
        .run()
        .await;
    }

    #[instrument(level = "debug", skip(self))]
    async fn run(&mut self) {
        debug!("Broker loop started.");

        loop {
            let next_timeout = self.handle_outdated_requests();

            self.send_waiting_requests().await;

            tokio::select! {
                _ = next_timeout.unwrap(), if next_timeout.is_some()=> {
                    // Handled in next loop iteration
                },
                signal =  self.api_request_receiver.recv() =>{
                    match signal {
                        Some(request) => {
                             self.api_call_queue.push_back(QueuedApiCall{
                                request,
                                timeout: Instant::now() +  self.options.request_timeout,
                            })
                        }
                        None => {
                            break;
                        }
                    }
                },
                (reconnect, connection_establish_task, received_data) = async {
                    match &mut self.loop_status.as_mut() {
                        BrokerLoopStatusInner::Disconnected { backoff_timeout} => {
                            sleep_until(*backoff_timeout).await;
                            (Some(()), None, None)
                        }
                        BrokerLoopStatusInner::Connecting{ connection_establish_task } => {
                            (None, Some(connection_establish_task.await),None)
                        }
                        BrokerLoopStatusInner::Connected{ connection, ..} => {
                            (None, None,  Some(connection.try_recv().await))
                        }
                    }
                } => {
                    match (reconnect, connection_establish_task, received_data)  {
                        (Some(()), None, None) => {
                             self.handle_connection_backoff_met();
                        }
                        (None, Some(resp), None) => {
                            self.handle_connection_established(resp);

                        }
                        (None, None, Some(received_data)) => {
                            self.handle_connection_data_received(received_data);

                        }
                        _ => unreachable!()
                    }
                }
            }
        }

        for call in self.api_call_queue.drain(..) {
            let _ = call
                .request
                .response_sender
                .send(Err(ApiCallError::BrokerConnectionClosed));
        }
        for (_correlation_id, (response_sender, _timeout)) in self.api_calls_in_transit.drain() {
            let _ = response_sender.send(Err(ApiCallError::BrokerConnectionClosed));
        }
        debug!("BrokerController main loop is closing");

        // TODO: Make sure it closes when broker is dropped - test?
    }

    #[instrument(level = "debug", skip(self))]
    fn handle_connection_backoff_met(&mut self) {
        debug_assert!(self.api_calls_in_transit.is_empty());
        let options = self.options.clone();
        let address = self.address.clone();
        let connection_task = timeout(options.connection_timeout, async move {
            let stream = TcpStream::connect(address)
                .await
                .map_err(BrokerConnectionInitializationError::ConnectionError)?;

            let buffer = BytesMut::with_capacity(options.buffer_size);
            let header = RequestHeader {
                client_id: options.client_name.to_owned(),
                ..Default::default()
            };

            let mut connection = BrokerConnection::new(buffer, stream, header);

            let api_versions_response = crate::broker::connection::call_api_inline(
                &mut connection,
                ApiVersionsRequest::default(),
                ApiVersionsRequest::get_min_supported_version(),
            )
            .await?;

            Ok((connection, api_versions_response))
        });

        self.loop_status.set(BrokerLoopStatusInner::Connecting {
            connection_establish_task: {
                tokio::spawn(async move {
                    connection_task
                        .await
                        .map_err(|_| BrokerConnectionInitializationError::ConnectionTimeoutReached)
                        .unwrap()
                })
            },
        });
    }

    #[instrument(level = "debug", skip(self))]
    fn handle_connection_data_received(
        &mut self,
        received_data: Option<Result<(ResponseHeader, BytesMut), ApiCallError>>,
    ) {
        match received_data {
            None => {
                // Not enough data transferred to deserialize full response
            }
            Some(response) => {
                match response {
                    Err(err) => {
                        debug!(?err, "Connection with kafka broker broken.");

                        for (_correlation_id, (resp_channel, _timeout)) in
                            self.api_calls_in_transit.drain()
                        {
                            let _ = resp_channel.send(Err(ApiCallError::BrokerConnectionClosed)); // Ignore error - client not waiting for ack/response
                        }
                        self.loop_status.set(BrokerLoopStatusInner::Disconnected {
                            backoff_timeout: Instant::now(),
                        }); // No delay for first try after connection was already established
                    }
                    Ok((header, payload)) => {
                        if let Some((response_channel, _timeout)) =
                            self.api_calls_in_transit.remove(&header.correlation_id)
                        {
                            let _ = response_channel.send(Ok(payload)); // Ignore error - client not waiting for ack/response
                        } else {
                            debug!(?header.correlation_id,"Received response for request that was already timed out on client side.");
                        }
                    }
                }
            }
        }
    }

    #[instrument(level = "debug", skip(self, resp))]
    fn handle_connection_established(
        &mut self,
        resp: Result<
            Result<(BrokerConnection, ApiVersionsResponse), BrokerConnectionInitializationError>,
            JoinError,
        >,
    ) {
        match resp {
            Ok(Ok((connection, api_versions))) => {
                self.loop_status
                    .set(BrokerLoopStatusInner::Connected { connection });
                let mut supported_apis = self.supported_api_versions.write().unwrap();
                *supported_apis = api_versions
                    .api_keys
                    .into_iter()
                    .map(|(k, v)| (k.index, v))
                    .collect();
                debug!("Connection with kafka broker established.");
            }
            Err(e) => {
                debug!(?e, "Connecting to kafka broker failed.");
                panic!("Error connecting to kafka broker {e:?}");
            }
            Ok(Err(e)) => {
                debug!(?e, "Connecting to kafka broker failed.");
                self.loop_status.set(BrokerLoopStatusInner::Disconnected {
                    backoff_timeout: Instant::now() + self.options.connection_retry_delay,
                });
            }
        }
    }

    #[instrument(level = "debug", skip(self))]
    async fn send_waiting_requests(&mut self) {
        if let BrokerLoopStatusInner::Connected { connection, .. } = &mut self.loop_status.as_mut()
        {
            while self.api_calls_in_transit.len() <= self.options.max_requests_per_connection {
                if let Some(call) = self.api_call_queue.pop_front() {
                    let result = connection
                        .send(
                            call.request.api_key,
                            call.request.api_version,
                            call.request.request,
                        )
                        .await;
                    match result {
                        Ok(correlation_id) => {
                            self.api_calls_in_transit.insert(
                                correlation_id,
                                (call.request.response_sender, call.timeout),
                            );
                        }
                        Err(err) => {
                            debug!(
                                ?err,
                                ?call.request.api_key,
                                ?call.request.api_version,
                                "Sending api request to kafka broker failed."
                            );
                            let _ = call.request.response_sender.send(Err(err)); // Ignore error - client not waiting for ack/response
                            for (_correlation_id, (resp_channel, _timeout)) in
                                self.api_calls_in_transit.drain()
                            {
                                let _ =
                                    resp_channel.send(Err(ApiCallError::BrokerConnectionClosed)); // Ignore error - client not waiting for ack/response
                            }
                            self.loop_status.set(BrokerLoopStatusInner::Disconnected {
                                backoff_timeout: Instant::now(),
                            }); // No delay for first try after connection was already
                            return;
                        }
                    }
                }
            }
        }
    }

    #[instrument(level = "debug", skip(self))]
    fn handle_outdated_requests(&mut self) -> Option<Sleep> {
        let mut next_timeout = loop {
            if let Some(QueuedApiCall { timeout, .. }) = self.api_call_queue.front() {
                if timeout <= &Instant::now() {
                    // Ignore error - request sender closed
                    _ = self
                        .api_call_queue
                        .pop_front()
                        .unwrap()
                        .request
                        .response_sender
                        .send(Err(ApiCallError::TimeoutReached));
                    continue;
                } else {
                    break Some(*timeout);
                }
            } else {
                break None;
            }
        };

        let mut timed_out = vec![];
        for (k, v) in self.api_calls_in_transit.iter() {
            if v.1 <= Instant::now() {
                timed_out.push(*k);
            } else {
                if let Some(t1) = next_timeout {
                    if v.1 < t1 {
                        next_timeout = Some(v.1);
                    }
                } else {
                    next_timeout = Some(v.1);
                }
            }
        }
        for k in timed_out {
            let call = self.api_calls_in_transit.remove(&k).unwrap();
            let _ = call.0.send(Err(ApiCallError::TimeoutReached));
        }

        next_timeout.map(sleep_until)
    }
}
