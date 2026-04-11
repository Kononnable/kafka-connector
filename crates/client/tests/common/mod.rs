#![allow(dead_code)]

use crate::common::cluster::SingleNodeCluster;
use crate::common::test_topic::TestTopic;
use kafka_connector_client::cluster::controller::ClusterController;
use kafka_connector_client::cluster::options::ClusterControllerOptions;
use kafka_connector_protocol::create_topics_request::CreatableTopic;
use std::sync::Arc;

pub mod cluster;
pub mod test_topic;

pub const KAFKA_TEST_BROKER_ADDR_1_HOST: &str = "localhost";
pub const KAFKA_TEST_BROKER_ADDR_1_PORT: u16 = 19092;
pub const KAFKA_TEST_BROKER_1_RACK: Option<&str> = Some("RACK_A");

pub const KAFKA_TEST_BROKER_ADDR_2_HOST: &str = "localhost";
pub const KAFKA_TEST_BROKER_ADDR_2_PORT: u16 = 29092;
pub const KAFKA_TEST_BROKER_2_RACK: Option<&str> = Some("RACK_B");

pub const KAFKA_TEST_BROKER_ADDR_3_HOST: &str = "localhost";
pub const KAFKA_TEST_BROKER_ADDR_3_PORT: u16 = 39092;
pub const KAFKA_TEST_BROKER_3_RACK: Option<&str> = None;

pub async fn create_single_node_with_single_topic()
-> (SingleNodeCluster, Arc<ClusterController>, TestTopic) {
    let kafka_cluster = SingleNodeCluster::new().await;

    let cluster = Arc::new(
        ClusterController::new(ClusterControllerOptions {
            bootstrap_servers: vec![(
                KAFKA_TEST_BROKER_ADDR_1_HOST.to_owned(),
                KAFKA_TEST_BROKER_ADDR_1_PORT,
            )],
            ..Default::default()
        })
        .await,
    );
    let topic = TestTopic::new(
        cluster.clone(),
        "topic",
        Some(CreatableTopic {
            num_partitions: 3,
            replication_factor: 1,
            ..Default::default()
        }),
    )
    .await;
    (kafka_cluster, cluster, topic)
}
