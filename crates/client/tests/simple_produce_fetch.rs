use crate::common::test_topic::TestTopic;
use crate::common::{KAFKA_TEST_BROKER_ADDR_1_HOST, KAFKA_TEST_BROKER_ADDR_1_PORT};
use kafka_connector_client::clients::consumer::client::KafkaConsumer;
use kafka_connector_client::clients::consumer::options::KafkaConsumerOptions;
use kafka_connector_client::clients::producer::client::KafkaProducer;
use kafka_connector_client::clients::producer::future_record::FutureRecord;
use kafka_connector_client::clients::producer::options::KafkaProducerOptions;
use kafka_connector_client::cluster::{
    controller::ClusterController, options::ClusterControllerOptions,
};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod common;

#[test_log::test(tokio::test)]
pub async fn main() {
    let cluster = ClusterController::new(ClusterControllerOptions {
        bootstrap_servers: vec![(
            KAFKA_TEST_BROKER_ADDR_1_HOST.to_owned(),
            KAFKA_TEST_BROKER_ADDR_1_PORT,
        )],
        ..Default::default()
    })
    .await;
    let cluster = Arc::new(cluster);
    let broker_list = cluster.get_broker_status_list();
    assert!(!broker_list.is_empty());

    let test_topic = TestTopic::new(cluster.clone(), "simple_produce_fetch", None).await;

    let mut consumer = KafkaConsumer::from_cluster_controller(
        cluster.clone(),
        KafkaConsumerOptions {
            topics: [test_topic.name().to_owned()].into(),
            ..Default::default()
        },
    );

    let producer =
        KafkaProducer::from_cluster_controller(cluster.clone(), KafkaProducerOptions::new());
    let key = "Key";
    let value = format!(
        "Time: {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    producer
        .send(FutureRecord::new(test_topic.name(), key, value.as_str()))
        .await
        .unwrap();

    let value2 = format!(
        "Time: {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    producer
        .send(FutureRecord::new(test_topic.name(), key, value2.as_str()))
        .await
        .unwrap();

    let record = consumer.recv().await;
    assert_eq!(String::from_utf8_lossy(record.key.as_slice()), key);
    assert_eq!(String::from_utf8_lossy(record.value.as_slice()), value);

    let record = consumer.recv().await;
    assert_eq!(String::from_utf8_lossy(record.key.as_slice()), key);
    assert_eq!(String::from_utf8_lossy(record.value.as_slice()), value2);

    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(consumer.try_recv().await.is_none());

    test_topic.delete().await;
}
