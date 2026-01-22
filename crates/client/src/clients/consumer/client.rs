use crate::clients::consumer::consumer_loop::ConsumerLoop;
use crate::cluster::controller::ClusterController;
use crate::cluster::error::ClusterControllerCreationError;
use crate::cluster::options::ClusterControllerOptions;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct KafkaConsumerOptions {
    pub topic: String, // TODO: temporary
}
pub struct KafkaConsumer {
    cluster: Arc<ClusterController>,
    options: KafkaConsumerOptions,
    record_channel: mpsc::Receiver<(Vec<u8>, Vec<u8>)>,
}

impl KafkaConsumer {
    pub async fn new(
        bootstrap_servers: Vec<impl ToSocketAddrs + Debug>,
        connection_options: ClusterControllerOptions,
        consumer_options: KafkaConsumerOptions,
    ) -> Result<KafkaConsumer, ClusterControllerCreationError> {
        let controller = ClusterController::new(bootstrap_servers, connection_options).await?;
        Ok(Self::from_cluster_controller(
            Arc::new(controller),
            consumer_options,
        ))
    }
    pub fn from_cluster_controller(
        controller: Arc<ClusterController>,
        consumer_options: KafkaConsumerOptions,
    ) -> KafkaConsumer {
        let (record_tx, record_rx) = mpsc::channel(1);
        tokio::spawn(ConsumerLoop::start(
            controller.clone(),
            consumer_options.clone(),
            record_tx,
        ));
        KafkaConsumer {
            cluster: controller,
            options: consumer_options,
            record_channel: record_rx,
        }
    }

    pub async fn recv(&mut self) -> Option<(Vec<u8>, Vec<u8>)> {
        self.record_channel.recv().await
    }
}
