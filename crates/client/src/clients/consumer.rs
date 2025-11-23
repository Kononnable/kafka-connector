use crate::cluster::controller::ClusterController;
use crate::cluster::error::ClusterControllerCreationError;
use crate::cluster::options::ClusterControllerOptions;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;

pub struct KafkaConsumerOptions {}
pub struct KafkaConsumer {
    cluster: Arc<ClusterController>,
    options: KafkaConsumerOptions,
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
        KafkaConsumer {
            cluster: controller,
            options: consumer_options,
        }
    }
}
