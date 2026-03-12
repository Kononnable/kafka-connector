use crate::common::cluster::SingleNodeCluster;
use crate::common::test_topic::TestTopic;
use crate::common::{KAFKA_TEST_BROKER_ADDR_1_HOST, KAFKA_TEST_BROKER_ADDR_1_PORT};
use kafka_connector_client::clients::consumer::client::KafkaConsumer;
use kafka_connector_client::clients::consumer::options::{KafkaConsumerOptions, OffsetReset};
use kafka_connector_client::clients::producer::client::KafkaProducer;
use kafka_connector_client::clients::producer::future_record::FutureRecord;
use kafka_connector_client::clients::producer::options::KafkaProducerOptions;
use kafka_connector_client::cluster::controller::ClusterController;
use kafka_connector_client::cluster::options::ClusterControllerOptions;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

mod common;

#[test_log::test(tokio::test)]
pub async fn consumes_messages_correctly_when_topic_is_created_while_consumer_is_running() {
    let _kafka_cluster = SingleNodeCluster::new().await;
    let topic_name = "non-existent".to_owned();

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

    let mut consumer = KafkaConsumer::from_cluster_controller(
        cluster.clone(),
        KafkaConsumerOptions {
            topics: [topic_name.clone()].into(),
            offset_reset: OffsetReset::Earliest,
            ..Default::default()
        },
    );

    tokio::time::sleep(Duration::from_millis(500)).await;

    let topic = TestTopic::new(cluster.clone(), &topic_name, None).await;

    let producer =
        KafkaProducer::from_cluster_controller(cluster.clone(), KafkaProducerOptions::new());

    let record = FutureRecord::new(&topic_name, vec![], vec![]);
    producer.send(record).await.unwrap();

    consumer.recv().await;

    topic.delete().await;
}

#[test_log::test(tokio::test)]
pub async fn start_consuming_messages_according_to_offset_reset_setting() {
    let _kafka_cluster = SingleNodeCluster::new().await;
    let topic_name = "test".to_owned();

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

    let topic = TestTopic::new(cluster.clone(), &topic_name, None).await;

    let producer =
        KafkaProducer::from_cluster_controller(cluster.clone(), KafkaProducerOptions::new());

    let mut record_1d = FutureRecord::new(&topic_name, "1d", vec![]);
    record_1d.timestamp = Some(SystemTime::now() - Duration::from_hours(24));
    producer.send(record_1d).await.unwrap();

    let mut record_1h = FutureRecord::new(&topic_name, "1h", vec![]);
    record_1h.timestamp = Some(SystemTime::now() - Duration::from_hours(1));
    producer.send(record_1h).await.unwrap();

    let mut consumer_earliest = KafkaConsumer::from_cluster_controller(
        cluster.clone(),
        KafkaConsumerOptions {
            topics: [topic_name.clone()].into(),
            offset_reset: OffsetReset::Earliest,
            ..Default::default()
        },
    );
    let mut consumer_from_now = KafkaConsumer::from_cluster_controller(
        cluster.clone(),
        KafkaConsumerOptions {
            topics: [topic_name.clone()].into(),
            offset_reset: OffsetReset::FromNow(Duration::from_hours(2)),
            ..Default::default()
        },
    );
    let mut consumer_latest = KafkaConsumer::from_cluster_controller(
        cluster.clone(),
        KafkaConsumerOptions {
            topics: [topic_name.clone()].into(),
            offset_reset: OffsetReset::Latest,
            ..Default::default()
        },
    );

    // Wait for consumers to fetch the data - try_recv is used
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert!(consumer_earliest.try_recv().is_some());
    assert!(consumer_earliest.try_recv().is_some());
    assert!(consumer_earliest.try_recv().is_none());

    assert!(consumer_from_now.try_recv().is_some());
    assert!(consumer_from_now.try_recv().is_none());

    assert!(consumer_latest.try_recv().is_none());

    topic.delete().await;
}
