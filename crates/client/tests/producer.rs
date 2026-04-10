mod common;

mod send {
    use crate::common::cluster::SingleNodeCluster;
    use crate::common::test_topic::TestTopic;
    use crate::common::{KAFKA_TEST_BROKER_ADDR_1_HOST, KAFKA_TEST_BROKER_ADDR_1_PORT};
    use kafka_connector_client::clients::producer::client::KafkaProducer;
    use kafka_connector_client::clients::producer::error::ProduceError;
    use kafka_connector_client::clients::producer::future_record::FutureRecord;
    use kafka_connector_client::clients::producer::options::{Acks, KafkaProducerOptions};
    use kafka_connector_client::cluster::controller::ClusterController;
    use kafka_connector_client::cluster::options::ClusterControllerOptions;
    use kafka_connector_protocol::create_topics_request::CreatableTopic;
    use std::pin::pin;
    use std::sync::Arc;
    use std::task::{Context, Poll, Waker};
    use std::time::Duration;

    #[test_log::test(tokio::test)]
    pub async fn returns_error_when_topic_does_not_exist() {
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

        let producer =
            KafkaProducer::from_cluster_controller(cluster.clone(), KafkaProducerOptions::new());

        let record = FutureRecord::new(&topic_name, vec![], vec![]);
        let result = producer.send(record).await;

        let is_error = if let ProduceError::TopicNotFound(topic) = result.err().unwrap()
            && topic == topic_name
        {
            true
        } else {
            false
        };
        assert!(is_error);
    }

    #[test_log::test(tokio::test)]
    pub async fn returns_error_when_forcing_message_on_partition_that_does_not_exist() {
        let _kafka_cluster = SingleNodeCluster::new().await;
        let topic_name = "topic".to_owned();

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
            &topic_name,
            Some(CreatableTopic {
                num_partitions: 3,
                replication_factor: 1,
                ..Default::default()
            }),
        )
        .await;

        let producer =
            KafkaProducer::from_cluster_controller(cluster.clone(), KafkaProducerOptions::new());

        let mut record = FutureRecord::new(&topic_name, vec![], vec![]);
        record.partition = Some(10);
        let result = producer.send(record).await;

        let is_error = if let ProduceError::PartitionNotFound(topic, partition) =
            dbg!(result).err().unwrap()
            && topic == topic_name
            && partition == 10
        {
            true
        } else {
            false
        };
        assert!(is_error);

        topic.delete().await;
    }

    #[test_log::test(tokio::test)]
    pub async fn produce_acks() {
        let _kafka_cluster = SingleNodeCluster::new().await;
        let topic_name = "topic".to_owned();

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
            &topic_name,
            Some(CreatableTopic {
                num_partitions: 3,
                replication_factor: 1,
                ..Default::default()
            }),
        )
        .await;

        let producer_no_ack = KafkaProducer::from_cluster_controller(
            cluster.clone(),
            KafkaProducerOptions {
                acks: Acks::NoAck,
                ..KafkaProducerOptions::new()
            },
        );
        let producer_leader = KafkaProducer::from_cluster_controller(
            cluster.clone(),
            KafkaProducerOptions {
                acks: Acks::Leader,
                ..KafkaProducerOptions::new()
            },
        );
        let producer_all = KafkaProducer::from_cluster_controller(
            cluster.clone(),
            KafkaProducerOptions {
                acks: Acks::All,
                ..KafkaProducerOptions::new()
            },
        );

        let record = FutureRecord::new(&topic_name, vec![], vec![]);
        let result_no_ack = producer_no_ack.send(record.clone()).await.unwrap();
        let result_leader = producer_leader.send(record.clone()).await.unwrap();
        let result_all = producer_all.send(record).await.unwrap();

        assert_eq!(result_no_ack.offset, -1);
        assert_eq!(result_leader.offset, 1);
        assert_eq!(result_all.offset, 2);

        topic.delete().await;
    }

    #[test_log::test(tokio::test)]
    pub async fn linger_groups_messages_into_batches() {
        let _kafka_cluster = SingleNodeCluster::new().await;
        let topic_name = "topic".to_owned();

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
            &topic_name,
            Some(CreatableTopic {
                num_partitions: 3,
                replication_factor: 1,
                ..Default::default()
            }),
        )
        .await;

        let producer = KafkaProducer::from_cluster_controller(
            cluster.clone(),
            KafkaProducerOptions {
                linger: Duration::from_millis(500),
                acks: Acks::NoAck,
                ..KafkaProducerOptions::new()
            },
        );

        let ctx = &mut Context::from_waker(Waker::noop());

        let record = FutureRecord::new(&topic_name, vec![], vec![]);
        let mut result_1 = pin!(producer.send(record.clone()));
        let mut result_2 = pin!(producer.send(record.clone()));
        tokio::time::sleep(Duration::from_millis(450)).await;
        assert!(matches!(result_1.as_mut().poll(ctx), Poll::Pending));
        assert!(matches!(result_2.as_mut().poll(ctx), Poll::Pending));
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(matches!(result_1.as_mut().poll(ctx), Poll::Ready(_)));
        assert!(matches!(result_2.as_mut().poll(ctx), Poll::Ready(_)));
        let mut result_3 = pin!(producer.send(record));
        tokio::time::sleep(Duration::from_millis(450)).await;
        assert!(matches!(result_3.as_mut().poll(ctx), Poll::Pending));
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(matches!(result_3.as_mut().poll(ctx), Poll::Ready(_)));

        topic.delete().await;
    }

    #[test_log::test(tokio::test)]
    pub async fn unsent_messages_on_producer_close_are_returned_with_error() {
        let _kafka_cluster = SingleNodeCluster::new().await;
        let topic_name = "topic".to_owned();

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
            &topic_name,
            Some(CreatableTopic {
                num_partitions: 3,
                replication_factor: 1,
                ..Default::default()
            }),
        )
        .await;

        let producer = KafkaProducer::from_cluster_controller(
            cluster.clone(),
            KafkaProducerOptions {
                linger: Duration::from_millis(100),
                acks: Acks::NoAck,
                ..KafkaProducerOptions::new()
            },
        );

        let record = FutureRecord::new(&topic_name, vec![], vec![]);
        let send_fut = producer.send(record);
        drop(producer);
        let result = send_fut.await;
        assert!(matches!(result, Err(ProduceError::ProducerClosed)));

        topic.delete().await;
    }
}
