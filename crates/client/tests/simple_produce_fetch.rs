use crate::common::KAFKA_TEST_BROKER_ADDR_1;
use crate::common::test_topic::TestTopic;
use kafka_connector_client::clients::consumer::client::{KafkaConsumer, KafkaConsumerOptions};
use kafka_connector_client::clients::producer::client::{KafkaProducer, KafkaProducerOptions};
use kafka_connector_client::clients::producer::future_record::FutureRecord;
use kafka_connector_client::cluster::{
    controller::ClusterController, options::ClusterControllerOptions,
};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

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

    let test_topic = TestTopic::new(cluster.clone(), "simple_produce_fetch", None).await;
    let producer = KafkaProducer::from_cluster_controller(cluster.clone(), KafkaProducerOptions {});
    let key = "Key";
    let value = format!(
        "Time: {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    producer
        .send(FutureRecord::new(test_topic.name(), key, value.as_str()))
        .await
        .unwrap();

    let mut consumer = KafkaConsumer::from_cluster_controller(
        cluster,
        KafkaConsumerOptions {
            topic: test_topic.name().to_owned(),
        },
    );
    let record = consumer.recv().await.unwrap();
    assert_eq!(String::from_utf8_lossy(record.0.as_slice()), key);
    assert_eq!(String::from_utf8_lossy(record.1.as_slice()), value);

    assert!(consumer.recv().await.is_none());

    test_topic.delete().await;
}
