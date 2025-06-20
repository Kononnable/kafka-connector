use crate::broker::connection::{BrokerConnection, BrokerConnectionInitializationError};
use crate::broker::controller::ResponseChannel;
use crate::cluster::error::ApiCallError;
use crate::{
    broker::{
        connection::fetch_initial_broker_list_from_broker,
        controller::{ApiRequestMessage, BrokerControllerStatus},
    },
    cluster::options::ClusterControllerOptions,
};
use kafka_connector_protocol::metadata_response::MetadataResponse;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing::{debug, instrument};

#[derive(Debug)]
pub enum BrokerLoopSignal {
    StatusRequest(oneshot::Sender<BrokerControllerStatus>),
    // TODO: Wait for no requests in transit - can be used in clean shutdown
}

struct QueuedApiCall {
    pub timeout: Instant,
    pub request: ApiRequestMessage,
}

enum BrokerLoopStatus {
    Connecting {
        connection_establish_task: JoinHandle<
            Result<(BrokerConnection, MetadataResponse), BrokerConnectionInitializationError>,
        >,
    },
    Connected {
        connection: BrokerConnection,
        api_calls_in_transit: HashMap<i32, ResponseChannel>,
    },
}

#[instrument(level = "debug", skip_all)]
pub async fn broker_loop(
    address: String,
    mut signal_receiver: mpsc::UnboundedReceiver<BrokerLoopSignal>,
    mut api_request_receiver: mpsc::UnboundedReceiver<ApiRequestMessage>,
    options: Arc<ClusterControllerOptions>,
    node_id: i32,
) {
    debug!(?node_id, ?address, "Broker loop started.");
    // TODO: Proper handle of retries - retries on client, not broker loop(?)
    // TODO: any cancel cannot be done mid-send (sending request partially will break a connection)
    // TODO: Trace logs

    let mut api_call_queue = VecDeque::new();

    let mut broker_loop_status = BrokerLoopStatus::Connecting {
        connection_establish_task: {
            let options = options.clone();
            let address = address.clone();
            tokio::spawn(async move {
                // TODO: normal method for broker connection, not fetch_initial_broker_list_from_broker
                // fetch broker supported api versions
                // should also fetch topic list periodically? (metadata refresh)
                fetch_initial_broker_list_from_broker(&options, &address).await
            })
        },
    };

    loop {
        // TODO: (?) user may specify custom timeout - making api_cal_queue not sorted (front, pop_front will be wrong)
        // TODO: join api calls in progress
        let next_timeout = if let Some(QueuedApiCall { timeout, .. }) = api_call_queue.front() {
            if timeout <= &Instant::now() {
                // Ignore error - request sender closed
                _ = api_call_queue
                    .pop_front()
                    .unwrap()
                    .request
                    .response_sender
                    .send(Err(ApiCallError::TimeoutReached));
                continue;
            } else {
                Some(tokio::time::sleep_until(*timeout))
            }
        } else {
            None
        };

        if let BrokerLoopStatus::Connected {
            connection,
            api_calls_in_transit,
        } = &mut broker_loop_status
        {
            while api_calls_in_transit.len() <= options.max_requests_per_connection {
                if let Some(call) = api_call_queue.pop_front() {
                    let correlation_id = connection
                        .send(
                            call.request.api_key,
                            call.request.api_version,
                            call.request.request,
                        )
                        .await
                        .unwrap(); // TODO:unwrap
                    api_calls_in_transit.insert(correlation_id, call.request.response_sender);
                }
            }
        }

        tokio::select! {
            _ = next_timeout.unwrap(), if next_timeout.is_some()=> {},
            signal = signal_receiver.recv() => {
                match signal{
                     Some(BrokerLoopSignal::StatusRequest(tx)) => {
                            let _ = tx.send(match &broker_loop_status {
                                BrokerLoopStatus::Connecting { .. } =>BrokerControllerStatus::Connecting,
                                BrokerLoopStatus::Connected { .. } => BrokerControllerStatus::Connected,
                            });
                        },
                        None => {
                            break;
                        }
                }
            }
            signal = api_request_receiver.recv() =>{
                match signal {
                    None => {
                        break;
                    }
                    Some(request) => {
                        api_call_queue.push_back(QueuedApiCall{
                            request,
                            timeout: Instant::now() + options.request_timeout,
                        })
                    }
                }
            },
            (connection_establish_task, received_data) = async {
                match &mut broker_loop_status {
                    BrokerLoopStatus::Connecting{ connection_establish_task } => {
                        (Some(connection_establish_task.await),None)
                    }
                    BrokerLoopStatus::Connected{ connection, api_calls_in_transit } => {
                        (None, Some((connection.try_recv().await , api_calls_in_transit)))
                    }}
            } => {
                match (connection_establish_task, received_data)  {
                    (Some(resp), None) => {
                        // TODO: Error handling - unwraps plus error codes(?)
                        let (connection,  metadata)  = resp.unwrap().unwrap();
                        broker_loop_status = BrokerLoopStatus::Connected {
                            connection,
                            api_calls_in_transit: HashMap::new()
                        };
                        debug!(?address, "Connection with kafka broker established.");
                    }
                    (None, Some((received_data,api_calls_in_transit))) => {
                        match received_data {
                            None => {
                                // Not enough data transferred to deserialize full response
                            }
                            Some(response) => {
                                match response {
                                    Err(_) => {
                                        debug!(?address, "Connection with kafka broker broken.");
                                        // TODO: Cancel all requests in progress

                                            // TODO: EOF or io error, send to all scheduled messages
                                            // TODO: Retries should be done in order, so should be handled here?
                                            // TODO: On break of broker connection cancel all ongoing api calls - return error
                                            //       (will be retried by another layer if needed) - this may be a problem for message ordering
                                            // TODO: Log error?
                                    }
                                    Ok((header,payload)) => {
                                        let response_channel = api_calls_in_transit.remove(&header.correlation_id).unwrap(); // TODO: Unrwap
                                        let _ = response_channel.send(Ok(payload)); // Ignore error - client not waiting for ack/response
                                    }
                                }
                            }
                        }
                    }
                    _ => unreachable!()
                }
            }
        }
    }
    // TODO: Close procedure
    // TODO: Cancel all requests
    // TODO: gracefully close tcp connection?
    debug!(node_id, "BrokerController main loop is closing");

    // TODO: Make sure it closes when broker is dropped - test?
}
