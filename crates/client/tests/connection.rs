use kafka_connector_client::cluster::controller::ClusterController;
use kafka_connector_client::cluster::options::ClusterControllerOptions;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::common::KAFKA_TEST_BROKER_ADDR_1;

mod common;

#[tokio::test]
pub async fn main() {
    // TODO" extract, consider if logging should be enabled by default in tests (probably yes, or through env filter)
    let my_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(my_subscriber).expect("setting tracing default failed");

    let cluster = ClusterController::new(
        vec![KAFKA_TEST_BROKER_ADDR_1],
        ClusterControllerOptions::default(),
    )
    .await
    .unwrap();
    let broker_list = cluster.get_broker_list();
    assert!(!broker_list.is_empty())
}
