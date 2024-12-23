use crate::{
    broker::{
        connection::fetch_initial_broker_list_from_broker,
        controller::{ApiRequestMessage, BrokerControllerStatus},
    },
    cluster::options::ClusterControllerOptions,
};
use rustc_hash::FxBuildHasher;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, instrument};

#[derive(Debug)]
pub enum BrokerLoopSignal {
    StatusRequest(oneshot::Sender<BrokerControllerStatus>),
    // TODO: Wait for no requests in transit - can be used in clean shutdown
}
#[instrument(level = "debug", skip_all)]
pub async fn broker_loop(
    address: String,
    mut signal_receiver: mpsc::UnboundedReceiver<BrokerLoopSignal>,
    mut api_request_receiver: mpsc::UnboundedReceiver<ApiRequestMessage>,
    options: Arc<ClusterControllerOptions>,
    node_id: i32,
) {
    // TODO: Proper handle of timeouts
    // TODO: Proper handle of retries
    // TODO: Where to handle api calls in transit limit(?)

    let mut calls_in_transit = HashMap::with_capacity_and_hasher(options.max_requests_per_connection, FxBuildHasher);

    loop {
        let options = options.clone();
        let address = address.clone();

        // TODO: some sort of circuit breaker
        let mut connection_establish_task =
            tokio::spawn(
                async move { fetch_initial_broker_list_from_broker(&options, &address).await },
            );

        let (mut connection, metadata) = loop {
            tokio::select! {
                signal = signal_receiver.recv() => {
                    match signal {
                        Some(BrokerLoopSignal::StatusRequest(tx)) => {
                            let _ = tx.send(BrokerControllerStatus::Connecting);
                        },
                        None => {
                            debug!(node_id, "BrokerController main loop is closing");
                            return;
                        }
                    }
                },
                res = &mut connection_establish_task => {
                    if let Ok(Ok(connection)) = res{
                        break connection;
                    }
                    // TODO: Log connection error (first panic, second error from function)
                }
            }
        };
        loop {
            tokio::select! {
                signal = signal_receiver.recv() => {
                    match signal {
                        None => {
                            debug!(node_id, "BrokerController main loop is closing");
                            return;
                        }
                        Some(BrokerLoopSignal::StatusRequest(tx)) => {
                            let _ = tx.send(BrokerControllerStatus::Connected);
                        },
                    }
                },
                signal = api_request_receiver.recv() =>{
                    match signal {
                        None => {
                            debug!(node_id, "BrokerController main loop is closing");
                            return;
                        }
                        Some(ApiRequestMessage{response_sender, api_key, api_version, request}) => {
                            if let Ok(correlation_id) = connection.send(api_key, api_version, request).await {
                                calls_in_transit.insert(correlation_id,response_sender);
                            } else {
                                todo_on_tcp_stream_error();
                            }
                        }
                    }
                },
                recv_result = connection.try_recv() => {
                    if let Some(recv_result) = recv_result {
                        match recv_result {
                            Ok((header, bytes)) => {
                                let channel = calls_in_transit.remove(&header.correlation_id).unwrap();
                                channel.send(Ok(bytes)).unwrap();
                            }
                            Err(_) => {
                                todo_on_tcp_stream_error();
                                break;
                            }
                        }
                    }
                }
            }
        }

        // TODO: Make sure it closes when broker is dropped - test?
    }
}

fn todo_on_tcp_stream_error() {
    // TODO: EOF or io error, send to all scheduled messages
    // TODO: Retries should be done in order, so should be handled here?
    // TODO: On break of broker connection cancel all ongoing api calls - return error
    //       (will be retried by another layer if needed) - this may be a problem for message ordering
    // TODO: Log error?
}
