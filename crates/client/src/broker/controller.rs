use crate::{
    broker::broker_loop::{broker_loop, BrokerLoopSignal},
    cluster::options::ClusterControllerOptions,
};
use bytes::BytesMut;
use kafka_connector_protocol::{ApiKey, ApiVersion};
use std::{future::Future, sync::Arc};
use thiserror::Error as DeriveError;
use tokio::sync::{mpsc, mpsc::UnboundedSender, oneshot};
use tracing::{error, instrument};

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ApiCallError {
    #[error("Broker connection closed before api response was received")]
    BrokerConnectionClosing,
    #[error("Broker communication error: {0}")]
    IoError(std::io::Error),
}

#[derive(Copy, Clone, Debug)]
pub enum BrokerControllerStatus {
    Connecting,
    Connected,
}

pub struct BrokerController {
    address: String,
    loop_tx: UnboundedSender<BrokerLoopSignal>,
    // TODO: Change type (make alias or more likely extract to struct)
    request_tx: mpsc::Sender<(
        oneshot::Sender<Result<BytesMut, ApiCallError>>,
        ApiKey,
        ApiVersion,
        BytesMut,
    )>,
    node_id: i32,
}

impl BrokerController {
    #[instrument(level = "debug")]
    pub fn new(
        address: String,
        options: Arc<ClusterControllerOptions>,
        node_id: i32,
    ) -> BrokerController {
        let (loop_tx, loop_rx) = mpsc::unbounded_channel();
        let (request_tx, request_rx) = mpsc::channel(10); // TODO: Size from options
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
            node_id,
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

    // TODO: Document when it may block
    // TODO: Should there be double future (async + returned future) - more robust - or just one - simpler api
    #[allow(clippy::async_yields_async)]
    #[instrument(level = "debug", skip_all)]
    pub async fn api_call(
        &self,
        key: ApiKey,
        version: ApiVersion,
        request: BytesMut,
    ) -> impl Future<Output = Result<BytesMut, ApiCallError>> {
        let (tx, rx) = oneshot::channel();
        self.request_tx
            .send((tx, key, version, request))
            .await
            .expect("Broker loop channel should be open");
        async {
            rx.await
                .map_err(|_| ApiCallError::BrokerConnectionClosing)?
        }
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }
}

// TODO: Tests
