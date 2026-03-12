use crate::common::cluster::ThreeNodeCluster;
use crate::common::{KAFKA_TEST_BROKER_ADDR_1_HOST, KAFKA_TEST_BROKER_ADDR_1_PORT};
use kafka_connector_client::cluster::controller::ClusterController;
use kafka_connector_client::cluster::error::ApiCallError;
use kafka_connector_client::cluster::options::ClusterControllerOptions;
use kafka_connector_protocol::metadata_request::MetadataRequest;
use std::sync::Arc;
use std::time::Duration;

mod common;

#[test_log::test(tokio::test)]
pub async fn fails_api_call_when_broker_offline_and_succeeds_when_broker_gets_back_online() {
    let kafka_cluster = ThreeNodeCluster::new().await;

    kafka_cluster.broker_2.pause().await.unwrap();

    let cluster = Arc::new(
        ClusterController::new(ClusterControllerOptions {
            bootstrap_servers: vec![(
                KAFKA_TEST_BROKER_ADDR_1_HOST.to_owned(),
                KAFKA_TEST_BROKER_ADDR_1_PORT,
            )],
            request_timeout: Duration::from_millis(1000),
            ..Default::default()
        })
        .await,
    );

    let result = cluster
        .make_api_call(
            2,
            MetadataRequest {
                topics: Some([].into()),
                allow_auto_topic_creation: true,
            },
            None,
        )
        .await;
    assert!(matches!(result, Err(ApiCallError::TimeoutReached)));

    kafka_cluster.broker_2.unpause().await.unwrap();
    let result = cluster
        .make_api_call(
            2,
            MetadataRequest {
                topics: Some([].into()),
                allow_auto_topic_creation: true,
            },
            None,
        )
        .await;
    assert!(result.is_ok());
}
