use crate::clients::producer::error::ProduceError;
use crate::clients::producer::future_record::FutureRecord;
use crate::clients::producer::options::KafkaProducerOptions;
use crate::clients::producer::partitioner::Partitioner;
use crate::clients::producer::producer_loop::{ProduceRequestMessage, ProducerLoop};
use crate::cluster::controller::ClusterController;
use crate::cluster::options::ClusterControllerOptions;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

#[derive(Debug)]
pub struct RecordAppend {
    pub partition: i32,
    pub offset: i64,
    pub timestamp: SystemTime,
}

pub struct KafkaProducer<P>
where
    P: Partitioner,
{
    _options: KafkaProducerOptions<P>,
    request_tx: UnboundedSender<ProduceRequestMessage>,
}

impl<P> KafkaProducer<P>
where
    P: Partitioner,
{
    pub async fn new(
        connection_options: ClusterControllerOptions,
        producer_options: KafkaProducerOptions<P>,
    ) -> KafkaProducer<P> {
        let controller = ClusterController::new(connection_options).await;
        Self::from_cluster_controller(Arc::new(controller), producer_options)
    }
    pub fn from_cluster_controller(
        controller: Arc<ClusterController>,
        producer_options: KafkaProducerOptions<P>,
    ) -> KafkaProducer<P> {
        let (tx, rx) = mpsc::unbounded_channel::<ProduceRequestMessage>();
        tokio::spawn(ProducerLoop::start(
            controller,
            producer_options.clone(),
            rx,
        ));
        KafkaProducer {
            _options: producer_options,
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

    use super::*;
    use crate::cluster::controller::ClusterController;
    use crate::test_utils::cluster_controller::initialize_as_single_broker_cluster;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::net::TcpListener;

    #[test_log::test(tokio::test)]
    async fn loop_closes_after_client_drops() {
        let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = server.local_addr().unwrap();
        let bootstrap_servers = vec![(local_addr.ip().to_string(), local_addr.port())];

        tokio::spawn(async move {
            initialize_as_single_broker_cluster(&server).await;
        });

        let cluster = Arc::new(
            ClusterController::new(ClusterControllerOptions {
                bootstrap_servers,
                ..Default::default()
            })
            .await,
        );

        let cluster_weak = Arc::downgrade(&cluster);

        let producer = KafkaProducer::from_cluster_controller(cluster, KafkaProducerOptions::new());
        drop(producer);

        // Wait for loop to exit
        tokio::time::sleep(Duration::from_millis(50)).await;

        assert!(cluster_weak.upgrade().is_none());
    }
}
