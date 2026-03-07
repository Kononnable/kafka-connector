use crate::cluster::controller::ClusterController;
use crate::cluster::options::ClusterControllerOptions;
use std::sync::Arc;

pub struct KafkaAdminOptions {}
pub struct KafkaAdmin {
    _cluster: Arc<ClusterController>,
    _options: KafkaAdminOptions,
}

impl KafkaAdmin {
    pub async fn new(
        connection_options: ClusterControllerOptions,
        admin_options: KafkaAdminOptions,
    ) -> KafkaAdmin {
        let controller = ClusterController::new(connection_options).await;
        Self::from_cluster_controller(Arc::new(controller), admin_options)
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
