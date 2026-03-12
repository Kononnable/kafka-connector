mod common;

mod send {
    use crate::common::cluster::SingleNodeCluster;
    use crate::common::test_topic::TestTopic;
    use crate::common::{KAFKA_TEST_BROKER_ADDR_1_HOST, KAFKA_TEST_BROKER_ADDR_1_PORT};
    use kafka_connector_client::clients::producer::client::KafkaProducer;
    use kafka_connector_client::clients::producer::error::ProduceError;
    use kafka_connector_client::clients::producer::future_record::FutureRecord;
    use kafka_connector_client::clients::producer::options::KafkaProducerOptions;
    use kafka_connector_client::cluster::controller::ClusterController;
    use kafka_connector_client::cluster::options::ClusterControllerOptions;
    use kafka_connector_protocol::create_topics_request::CreatableTopic;
    use std::sync::Arc;

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
}
