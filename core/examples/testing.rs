use kafka_connector::kafka_client::KafkaClient;
use kafka_connector::protocol;
use protocol::{
    api::metadata::MetadataRequestTopics9, custom_types::compact_string::CompactString,
    optional::Optional,
};

const BROKER: &str = "127.0.0.1:9092";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let kafka_client = KafkaClient::new(BROKER, "kafka-connector-test").await?;
    let broker_client = kafka_client.clients.first().unwrap();

    let metadata_request = protocol::api::metadata::MetadataRequest {
        topics: vec![MetadataRequestTopics9 {
            name: "test".to_owned().into(),
        }],
        allow_auto_topic_creation: Optional::Some(false),
        include_cluster_authorized_operations: Optional::Some(false),
        include_topic_authorized_operations: Optional::Some(false),
    };
    // let fetch_request = protocol::api::fetch::FetchRequest {
    //     forgotten_topics_data: (),
    //     isolation_level: 0,
    //     replica_id: -1,
    //     max_wait_ms: 1000,
    //     min_bytes: 1,
    //     max_bytes: 32000,
    //     session_id: (),
    //     session_epoch: (),
    //     topics: (),
    //     rack_id: (),
    // };
    Ok(())
}
