use kafka_connector_client::cluster::controller::ClusterController;
use kafka_connector_client::cluster::options::ClusterControllerOptions;

use crate::common::KAFKA_TEST_BROKER_ADDR_1;

mod common;

#[tokio::test]
pub async fn main() {
    let cluster = ClusterController::new(vec![KAFKA_TEST_BROKER_ADDR_1], ClusterControllerOptions {})
        .await
        .unwrap();
    let broker_list = cluster.get_broker_list();
    assert!(!broker_list.is_empty())
}
