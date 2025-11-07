use crate::broker::broker_loop::BrokerLoop;
use crate::cluster::{error::ApiCallError, options::ClusterControllerOptions};
use bytes::BytesMut;
use indexmap::IndexMap;
use kafka_connector_protocol::api_versions_response::ApiVersionsResponseKey;
use kafka_connector_protocol::{ApiKey, ApiRequest, ApiResponse, ApiVersion};
use std::sync::RwLock;
use std::{
    future::Future,
    sync::{Arc, Mutex},
};
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

#[derive(Clone, Debug)]
pub enum BrokerControllerStatus {
    Connecting,
    Connected,
    Disconnected,
}
// TODO: Extract to separate file?
pub(super) struct ApiRequestMessage {
    pub response_sender: ResponseChannel,
    pub api_key: ApiKey,
    pub api_version: ApiVersion,
    pub request: BytesMut,
}
pub(super) type ResponseChannel = oneshot::Sender<Result<BytesMut, ApiCallError>>;

pub struct BrokerController {
    address: String,
    request_tx: mpsc::UnboundedSender<ApiRequestMessage>,
    buffer: Mutex<BytesMut>,
    options: Arc<ClusterControllerOptions>,
    _node_id: i32,
    pub(crate) supported_api_versions: Arc<RwLock<IndexMap<i16, ApiVersionsResponseKey>>>, // TODO: pub crate for api(?)
    status: Arc<RwLock<BrokerControllerStatus>>,
}

impl BrokerController {
    // TODO: Assumption - until connected supported api versions are the same as on other nodes (copy from initial connection to cluster)
    // TODO: Document parameters - parent
    // TODO: metadata refresh
    #[instrument(level = "debug")]
    pub fn new(
        address: String,
        options: &Arc<ClusterControllerOptions>,
        node_id: i32,
        parent_supported_apis: IndexMap<i16, ApiVersionsResponseKey>,
    ) -> BrokerController {
        let (request_tx, request_rx) = mpsc::unbounded_channel();
        let supported_api_versions = Arc::new(RwLock::new(parent_supported_apis));
        let status = Arc::new(RwLock::new(BrokerControllerStatus::Disconnected));
        tokio::spawn(BrokerLoop::start(
            address.clone(),
            request_rx,
            options.clone(),
            node_id,
            supported_api_versions.clone(),
            status.clone(),
        ));
        BrokerController {
            address,
            request_tx,
            buffer: Mutex::new(BytesMut::with_capacity(options.buffer_size)),
            options: options.clone(),
            _node_id: node_id,
            supported_api_versions,
            status,
        }
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn get_status(&self) -> BrokerControllerStatus {
        self.status.read().expect("Poisoned lock").clone()
    }

    // TODO: Test
    #[instrument(level = "debug", skip_all)]
    pub fn make_api_call<R: ApiRequest>(
        &self,
        version: ApiVersion,
        request: R,
    ) -> impl Future<Output = Result<R::Response, ApiCallError>> + use<R> {
        let (tx, rx) = oneshot::channel();

        let mut buffer = self.buffer.lock().expect("Poisoned lock");
        if buffer.capacity() < self.options.buffer_size {
            buffer.reserve(self.options.buffer_size);
        }
        let serialization_result = request.serialize(version, &mut buffer).map(|_| {
            let request_buf = &mut (*buffer);
            self.request_tx
                .send(ApiRequestMessage {
                    response_sender: tx,
                    api_key: R::get_api_key(),
                    api_version: version,
                    request: request_buf.split_to(request_buf.len()),
                })
                .expect("Broker loop channel should be open");
        });

        async move {
            if let Err(err) = serialization_result {
                Result::<R::Response, ApiCallError>::Err(ApiCallError::SerializationError(err))
            } else {
                rx.await
                    .map_err(|_| ApiCallError::BrokerConnectionClosed)?
                    .map(|mut resp| R::Response::deserialize(version, &mut resp))
            }
        }
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }
}
