use crate::common::cluster::{SingleNodeCluster, ThreeNodeCluster};
use crate::common::test_topic::TestTopic;
use crate::common::{
    KAFKA_TEST_BROKER_1_RACK, KAFKA_TEST_BROKER_2_RACK, KAFKA_TEST_BROKER_3_RACK,
    KAFKA_TEST_BROKER_ADDR_1_HOST, KAFKA_TEST_BROKER_ADDR_1_PORT, KAFKA_TEST_BROKER_ADDR_2_HOST,
    KAFKA_TEST_BROKER_ADDR_2_PORT, KAFKA_TEST_BROKER_ADDR_3_HOST, KAFKA_TEST_BROKER_ADDR_3_PORT,
};
use kafka_connector_client::cluster::controller::{ClusterController, ForceRefresh};
use kafka_connector_client::cluster::options::{
    ClusterControllerAdvancedOptions, ClusterControllerOptions,
};
use std::sync::Arc;
use std::time::Duration;

mod common;

#[test_log::test(tokio::test)]
pub async fn bootstrap_with_single_address_connects_with_all_from_metadata() {
    let _kafka_cluster = ThreeNodeCluster::new().await;
    let cluster = ClusterController::new(ClusterControllerOptions {
        bootstrap_servers: vec![("127.0.0.1".to_owned(), KAFKA_TEST_BROKER_ADDR_1_PORT)],
        ..Default::default()
    })
    .await;

    let broker_list = cluster.get_broker_list();
    assert_eq!(broker_list.len(), 3);

    let metadata = &broker_list.get(&1).unwrap().0;
    assert_eq!(metadata.host, KAFKA_TEST_BROKER_ADDR_1_HOST);
    assert_eq!(metadata.port, KAFKA_TEST_BROKER_ADDR_1_PORT as i32);
    assert_eq!(
        metadata.rack,
        KAFKA_TEST_BROKER_1_RACK.map(ToOwned::to_owned)
    );

    let metadata = &broker_list.get(&2).unwrap().0;
    assert_eq!(metadata.host, KAFKA_TEST_BROKER_ADDR_2_HOST);
    assert_eq!(metadata.port, KAFKA_TEST_BROKER_ADDR_2_PORT as i32);
    assert_eq!(
        metadata.rack,
        KAFKA_TEST_BROKER_2_RACK.map(ToOwned::to_owned)
    );

    let metadata = &broker_list.get(&3).unwrap().0;
    assert_eq!(metadata.host, KAFKA_TEST_BROKER_ADDR_3_HOST);
    assert_eq!(metadata.port, KAFKA_TEST_BROKER_ADDR_3_PORT as i32);
    assert_eq!(
        metadata.rack,
        KAFKA_TEST_BROKER_3_RACK.map(ToOwned::to_owned)
    );
}

#[test_log::test(tokio::test)]
pub async fn cluster_metadata_caches_correctly() {
    let _kafka_cluster = SingleNodeCluster::new().await;
    let topic_name = "test_topic";
    let metadata_refresh_interval = Duration::from_millis(1_000);

    let controller = Arc::new(
        ClusterController::new(ClusterControllerOptions {
            bootstrap_servers: vec![(
                KAFKA_TEST_BROKER_ADDR_1_HOST.to_owned(),
                KAFKA_TEST_BROKER_ADDR_1_PORT,
            )],
            advanced: ClusterControllerAdvancedOptions {
                metadata_refresh_interval,
                ..Default::default()
            },
            ..Default::default()
        })
        .await,
    );

    let metadata_entry_exists = || async {
        controller
            .get_metadata([topic_name.to_owned()].into(), ForceRefresh::No)
            .await
            .unwrap()
            .get(topic_name)
            .unwrap()
            .error_code
            .is_none()
    };
    assert!(!metadata_entry_exists().await);
    let test_topic = TestTopic::new(controller.clone(), topic_name, None).await;
    assert!(metadata_entry_exists().await);
    test_topic.delete().await;
    assert!(metadata_entry_exists().await);
    // Wait for cache to be invalidated
    tokio::time::sleep(metadata_refresh_interval).await;
    assert!(!metadata_entry_exists().await);
}
