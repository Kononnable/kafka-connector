use crate::clients::producer::error::ProduceError;
use crate::clients::producer::future_record::FutureRecord;
use crate::clients::producer::producer_loop::{ProduceRequestMessage, ProducerLoop};
use crate::cluster::controller::ClusterController;
use crate::cluster::error::ClusterControllerCreationError;
use crate::cluster::options::ClusterControllerOptions;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

#[derive(Debug)]
pub struct RecordAppend {
    pub partition: i32,
    pub offset: i64,
    pub timestamp: SystemTime,
}

#[derive(Clone, Debug, Default)]
pub struct KafkaProducerOptions {}
pub struct KafkaProducer {
    options: KafkaProducerOptions,
    request_tx: UnboundedSender<ProduceRequestMessage>,
}

impl KafkaProducer {
    pub async fn new(
        bootstrap_servers: Vec<impl ToSocketAddrs + Debug>,
        connection_options: ClusterControllerOptions,
        producer_options: KafkaProducerOptions,
    ) -> Result<KafkaProducer, ClusterControllerCreationError> {
        let controller = ClusterController::new(bootstrap_servers, connection_options).await?;
        Ok(Self::from_cluster_controller(
            Arc::new(controller),
            producer_options,
        ))
    }
    pub fn from_cluster_controller(
        controller: Arc<ClusterController>,
        producer_options: KafkaProducerOptions,
    ) -> KafkaProducer {
        let (tx, rx) = mpsc::unbounded_channel::<ProduceRequestMessage>();
        tokio::spawn(ProducerLoop::start(
            controller,
            producer_options.clone(),
            rx,
        ));
        KafkaProducer {
            options: producer_options,
            request_tx: tx,
        }
    }

    // TODO: retries (+ backoff?)
    // TODO: should return type be Future or something hiding it? must_use may not be desired in this case + document it
    #[instrument(level = "debug", skip_all)]
    pub fn send<R>(&self, record: R) -> impl Future<Output = Result<RecordAppend, ProduceError>>
    where
        R: Into<FutureRecord> + Debug,
    {
        let (tx, rx) = oneshot::channel();
        self.request_tx
            .send((record.into(), tx))
            .expect("Producer loop dropped while producer is still alive");
        async move {
            rx.await
                .expect("Producer loop dropped while producer is still alive")
        }
    }
}
#[cfg(test)]
mod tests {
    // TODO: Test - topic does not exist
    // TODO: Test - partition does not exist (partition set directly in FutureRecord)
    // TODO: Test - multiple records send at once, some fail
    // TODO: Test - batching
}
