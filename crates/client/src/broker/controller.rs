use crate::{
    broker::broker_loop::{broker_loop, BrokerLoopSignal},
    cluster::{error::ApiCallError, options::ClusterControllerOptions},
};
use bytes::BytesMut;
use kafka_connector_protocol::{ApiKey, ApiRequest, ApiResponse, ApiVersion};
use std::{future::Future, sync::Arc};
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

#[derive(Copy, Clone, Debug)]
pub enum BrokerControllerStatus {
    Connecting,
    Connected,
}
// TODO: Extract to separate file?
pub(super) struct ApiRequestMessage {
    pub response_sender: oneshot::Sender<Result<BytesMut, ApiCallError>>,
    pub api_key: ApiKey,
    pub api_version: ApiVersion,
    pub request: BytesMut,
}

pub struct BrokerController {
    address: String,
    loop_tx: mpsc::UnboundedSender<BrokerLoopSignal>,
    request_tx: mpsc::UnboundedSender<ApiRequestMessage>,
    _node_id: i32,
}

impl BrokerController {
    #[instrument(level = "debug")]
    pub fn new(
        address: String,
        options: Arc<ClusterControllerOptions>,
        node_id: i32,
    ) -> BrokerController {
        let (loop_tx, loop_rx) = mpsc::unbounded_channel();
        let (request_tx, request_rx) = mpsc::unbounded_channel();
        tokio::spawn(broker_loop(
            address.clone(),
            loop_rx,
            request_rx,
            options,
            node_id,
        ));
        BrokerController {
            address,
            loop_tx,
            request_tx,
            _node_id: node_id,
        }
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn get_status(&self) -> BrokerControllerStatus {
        let (tx, rx) = oneshot::channel();
        self.loop_tx
            .send(BrokerLoopSignal::StatusRequest(tx))
            .expect("Broker loop channel should be open");
        rx.await.expect("Broker loop channel should be open")
    }

    #[instrument(level = "debug", skip_all)]
    pub fn make_api_call<R: ApiRequest>(
        &self,
        version: ApiVersion,
        request: R,
    ) -> impl Future<Output = Result<R::Response, ApiCallError>> {
        let (tx, rx) = oneshot::channel();
        // TODO: reuse bytes, set the size
        let mut serialized_request = BytesMut::with_capacity(1_024);
        let serialization_result = request
            .serialize(version, &mut serialized_request)
            .map(|_| {
                self.request_tx
                    .send(ApiRequestMessage {
                        response_sender: tx,
                        api_key: R::get_api_key(),
                        api_version: version,
                        request: serialized_request,
                    })
                    .expect("Broker loop channel should be open");
            });

        async move {
            if let Err(err) = serialization_result {
                Result::<R::Response, ApiCallError>::Err(ApiCallError::SerializationError(err))
            } else {
                rx.await
                    .map_err(|_| ApiCallError::BrokerConnectionClosing)?
                    .map(|mut resp| R::Response::deserialize(version, &mut resp))
            }
        }
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }
}

// TODO: Tests
