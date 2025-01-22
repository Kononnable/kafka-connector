use kafka_connector_client::cluster::{
    controller::ClusterController, options::ClusterControllerOptions,
};

use crate::common::KAFKA_TEST_BROKER_ADDR_1;

mod common;

#[test_log::test(tokio::test)]
pub async fn main() {
    let cluster = ClusterController::new(
        vec![KAFKA_TEST_BROKER_ADDR_1],
        ClusterControllerOptions::default(),
    )
    .await
    .unwrap();
    let broker_list = cluster.get_broker_list().await;
    assert!(!broker_list.is_empty())
}
