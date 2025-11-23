use crate::cluster::controller::ClusterController;
use crate::cluster::error::ClusterControllerCreationError;
use crate::cluster::options::ClusterControllerOptions;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;

pub struct KafkaProducerOptions {}
pub struct KafkaProducer {
    cluster: Arc<ClusterController>,
    options: KafkaProducerOptions,
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
        KafkaProducer {
            cluster: controller,
            options: producer_options,
        }
    }
}
