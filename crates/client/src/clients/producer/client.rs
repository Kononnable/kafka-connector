use crate::clients::producer::future_record::FutureRecord;
use crate::clients::producer::producer_loop::ProducerLoop;
use crate::cluster::controller::ClusterController;
use crate::cluster::error::ClusterControllerCreationError;
use crate::cluster::options::ClusterControllerOptions;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

pub(super) struct ProduceRequestMessage {
    pub response_sender: oneshot::Sender<()>,
    pub records: Vec<FutureRecord>,
}

#[derive(Clone, Debug, Default)]
pub struct KafkaProducerOptions {}
pub struct KafkaProducer {
    cluster: Arc<ClusterController>,
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
        let (request_tx, request_rx) = mpsc::unbounded_channel::<ProduceRequestMessage>();
        tokio::spawn(ProducerLoop::start(
            controller.clone(),
            producer_options.clone(),
            request_rx,
        ));
        KafkaProducer {
            cluster: controller,
            options: producer_options,
            request_tx,
        }
    }

    // TODO: send_multiple<R>
    #[instrument(level = "debug", skip_all)]
    pub fn send<R>(&self, record: R) -> impl Future<Output = Result<(), ()>>
    where
        R: Into<FutureRecord> + Debug,
    {
        let (tx, rx) = oneshot::channel();
        // TODO: Cleanup, error handling
        self.request_tx
            .send(ProduceRequestMessage {
                response_sender: tx,
                records: vec![record.into()],
            })
            .unwrap();
        async move {
            // TODO:
            rx.await.unwrap();
            Ok(())
        }
    }
}
