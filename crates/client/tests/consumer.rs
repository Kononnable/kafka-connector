use crate::common::cluster::SingleNodeCluster;
use crate::common::test_topic::TestTopic;
use crate::common::{
    KAFKA_TEST_BROKER_ADDR_1_HOST, KAFKA_TEST_BROKER_ADDR_1_PORT,
    create_single_node_with_single_topic,
};
use kafka_connector_client::clients::consumer::client::KafkaConsumer;
use kafka_connector_client::clients::consumer::options::{KafkaConsumerOptions, OffsetReset};
use kafka_connector_client::clients::producer::client::KafkaProducer;
use kafka_connector_client::clients::producer::future_record::FutureRecord;
use kafka_connector_client::clients::producer::options::KafkaProducerOptions;
use kafka_connector_client::cluster::controller::ClusterController;
use kafka_connector_client::cluster::options::ClusterControllerOptions;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::time::timeout;

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
    let (_kafka_cluster, cluster, topic) = create_single_node_with_single_topic().await;

    let producer =
        KafkaProducer::from_cluster_controller(cluster.clone(), KafkaProducerOptions::new());

    let mut record_1d = FutureRecord::new(topic.name(), "1d", vec![]);
    record_1d.timestamp = Some(SystemTime::now() - Duration::from_hours(24));
    producer.send(record_1d).await.unwrap();

    let mut record_1h = FutureRecord::new(topic.name(), "1h", vec![]);
    record_1h.timestamp = Some(SystemTime::now() - Duration::from_hours(1));
    producer.send(record_1h).await.unwrap();

    let mut consumer_earliest = KafkaConsumer::from_cluster_controller(
        cluster.clone(),
        KafkaConsumerOptions {
            topics: [topic.name().to_owned()].into(),
            offset_reset: OffsetReset::Earliest,
            ..Default::default()
        },
    );
    let mut consumer_from_now = KafkaConsumer::from_cluster_controller(
        cluster.clone(),
        KafkaConsumerOptions {
            topics: [topic.name().to_owned()].into(),
            offset_reset: OffsetReset::FromNow(Duration::from_hours(2)),
            ..Default::default()
        },
    );
    let mut consumer_latest = KafkaConsumer::from_cluster_controller(
        cluster.clone(),
        KafkaConsumerOptions {
            topics: [topic.name().to_owned()].into(),
            offset_reset: OffsetReset::Latest,
            ..Default::default()
        },
    );

    tokio::time::sleep(Duration::from_millis(500)).await;

    assert!(
        timeout(Duration::from_millis(10), consumer_earliest.recv())
            .await
            .is_ok()
    );
    assert!(
        timeout(Duration::from_millis(10), consumer_earliest.recv())
            .await
            .is_ok()
    );
    assert!(
        timeout(Duration::from_millis(10), consumer_earliest.recv())
            .await
            .is_err()
    );

    assert!(
        timeout(Duration::from_millis(10), consumer_from_now.recv())
            .await
            .is_ok()
    );
    assert!(
        timeout(Duration::from_millis(10), consumer_from_now.recv())
            .await
            .is_err()
    );

    assert!(
        timeout(Duration::from_millis(10), consumer_latest.recv())
            .await
            .is_err()
    );

    topic.delete().await;
}

mod consumer_group {
    use crate::common::{
        KAFKA_TEST_BROKER_ADDR_1_HOST, KAFKA_TEST_BROKER_ADDR_1_PORT,
        create_single_node_with_single_topic,
    };
    use kafka_connector_client::clients::consumer::client::KafkaConsumer;
    use kafka_connector_client::clients::consumer::options::KafkaConsumerOptions;
    use kafka_connector_client::clients::producer::client::KafkaProducer;
    use kafka_connector_client::clients::producer::future_record::FutureRecord;
    use kafka_connector_client::clients::producer::options::KafkaProducerOptions;
    use kafka_connector_client::cluster::controller::ClusterController;
    use kafka_connector_client::cluster::options::ClusterControllerOptions;
    use std::sync::Arc;
    use std::time::Duration;

    #[test_log::test(tokio::test)]
    pub async fn consumers_share_partitions() {
        let (_kafka_cluster, _cluster, topic) = create_single_node_with_single_topic().await;
        // TODO: Multiple cluster controller needed - brokers tracks consumers by connection, not just member_id
        let cluster1 = Arc::new(
            ClusterController::new(ClusterControllerOptions {
                bootstrap_servers: vec![(
                    KAFKA_TEST_BROKER_ADDR_1_HOST.to_owned(),
                    KAFKA_TEST_BROKER_ADDR_1_PORT,
                )],
                client_name: "AAA".to_owned(),
                ..Default::default()
            })
            .await,
        );
        let cluster2 = Arc::new(
            ClusterController::new(ClusterControllerOptions {
                bootstrap_servers: vec![(
                    KAFKA_TEST_BROKER_ADDR_1_HOST.to_owned(),
                    KAFKA_TEST_BROKER_ADDR_1_PORT,
                )],
                client_name: "BBB".to_owned(),
                ..Default::default()
            })
            .await,
        );
        let cluster3 = Arc::new(
            ClusterController::new(ClusterControllerOptions {
                bootstrap_servers: vec![(
                    KAFKA_TEST_BROKER_ADDR_1_HOST.to_owned(),
                    KAFKA_TEST_BROKER_ADDR_1_PORT,
                )],
                client_name: "CCC".to_owned(),
                ..Default::default()
            })
            .await,
        );
        let group_id = Some("consumer group".to_owned());

        let producer =
            KafkaProducer::from_cluster_controller(cluster1.clone(), KafkaProducerOptions::new());

        let mut record_1 = FutureRecord::new("topic", "0", vec![]);
        record_1.partition = Some(0);
        let mut record_2 = FutureRecord::new("topic", "1", vec![]);
        record_2.partition = Some(1);
        let mut record_3 = FutureRecord::new("topic", "2", vec![]);
        record_3.partition = Some(2);

        let mut consumer_1 = KafkaConsumer::from_cluster_controller(
            cluster1,
            KafkaConsumerOptions {
                topics: ["topic".to_owned()].into(),
                group_id: group_id.clone(),
                ..Default::default()
            },
        );
        let mut consumer_2 = KafkaConsumer::from_cluster_controller(
            cluster2,
            KafkaConsumerOptions {
                topics: ["topic".to_owned()].into(),
                group_id: group_id.clone(),
                ..Default::default()
            },
        );
        let mut consumer_3 = KafkaConsumer::from_cluster_controller(
            cluster3,
            KafkaConsumerOptions {
                topics: ["topic".to_owned()].into(),
                group_id: group_id.clone(),
                ..Default::default()
            },
        );

        // Wait for consumer group to be fully created, rebalanced (multiple consumers)
        tokio::time::sleep(Duration::from_millis(10_000)).await;

        // TODO: allow _ binding in produce (don't return future directly)
        drop(producer.send(record_1));
        drop(producer.send(record_2));
        drop(producer.send(record_3));

        let mut records = Vec::with_capacity(3);
        records.push(consumer_1.recv().await);
        records.push(consumer_2.recv().await);
        records.push(consumer_3.recv().await);

        let keys = records
            .into_iter()
            .map(|r| String::from_utf8(r.key).unwrap())
            .collect::<Vec<_>>();
        assert!(keys.contains(&"0".to_string()));
        assert!(keys.contains(&"1".to_string()));
        assert!(keys.contains(&"2".to_string()));

        assert!(consumer_1.try_recv().is_none());
        assert!(consumer_2.try_recv().is_none());
        assert!(consumer_3.try_recv().is_none());

        topic.delete().await;
    }
}
