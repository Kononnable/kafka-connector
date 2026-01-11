use crate::common::KAFKA_TEST_BROKER_ADDR_1;
use kafka_connector_client::clients::producer::client::{KafkaProducer, KafkaProducerOptions};
use kafka_connector_client::clients::producer::future_record::FutureRecord;
use kafka_connector_client::cluster::{
    controller::ClusterController, options::ClusterControllerOptions,
};
use std::sync::Arc;

mod common;

#[test_log::test(tokio::test)]
pub async fn main() {
    let cluster = ClusterController::new(
        vec![KAFKA_TEST_BROKER_ADDR_1],
        ClusterControllerOptions::default(),
    )
    .await
    .unwrap();
    let cluster = Arc::new(cluster);
    let broker_list = cluster.get_broker_list().await;
    assert!(!broker_list.is_empty());

    //// Produce, no longer pure connection test

    // TODO: make sure topic exist/create

    let producer = KafkaProducer::from_cluster_controller(cluster, KafkaProducerOptions {});
    producer
        .send(FutureRecord::new("AABBS", "Key", "Val")) // TODO: randomize topic name(?)
        .await
        .unwrap();

    // TODO: consumer

    // TODO: cleanup - remove topic (?)
}
