use crate::cluster::controller::ClusterController;
use crate::cluster::error::ClusterControllerCreationError;
use crate::cluster::options::ClusterControllerOptions;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;

pub struct KafkaAdminOptions {}
pub struct KafkaAdmin {
    _cluster: Arc<ClusterController>,
    _options: KafkaAdminOptions,
}

impl KafkaAdmin {
    pub async fn new(
        bootstrap_servers: Vec<impl ToSocketAddrs + Debug>,
        connection_options: ClusterControllerOptions,
        admin_options: KafkaAdminOptions,
    ) -> Result<KafkaAdmin, ClusterControllerCreationError> {
        let controller = ClusterController::new(bootstrap_servers, connection_options).await?;
        Ok(Self::from_cluster_controller(
            Arc::new(controller),
            admin_options,
        ))
    }
    pub fn from_cluster_controller(
        controller: Arc<ClusterController>,
        admin_options: KafkaAdminOptions,
    ) -> KafkaAdmin {
        KafkaAdmin {
            _cluster: controller,
            _options: admin_options,
        }
    }
}
