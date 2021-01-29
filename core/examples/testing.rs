use kafka_connector::kafka_client::{BrokerClient, KafkaClient};
use kafka_connector::protocol;
use protocol::{
    api::{metadata::MetadataRequestTopics9, ApiNumbers},
    custom_types::optional::Optional,
};

const BROKER: &str = "127.0.0.1:9092";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut broker = BrokerClient::new(BROKER, "kafka-connector-test".to_owned()).await?;

    let metadata_request = protocol::api::metadata::MetadataRequest {
        topics: vec![MetadataRequestTopics9 {
            name: "test".to_owned().into(),
        }],
        allow_auto_topic_creation: Optional::None,
        include_cluster_authorized_operations: Optional::None,
        include_topic_authorized_operations: Optional::None,
    };
    let supported_version = broker
        .supported_versions
        .get(&(ApiNumbers::Metadata as i16));
    println!("supported_versions {:?}", supported_version);
    broker
        .run_api_call(metadata_request.clone(), Some(0))
        .await?;
    broker
        .run_api_call(metadata_request.clone(), Some(1))
        .await?;
    broker
        .run_api_call(metadata_request.clone(), Some(2))
        .await?;
    broker
        .run_api_call(metadata_request.clone(), Some(3))
        .await?;
    broker
        .run_api_call(metadata_request.clone(), Some(4))
        .await?;
    broker
        .run_api_call(metadata_request.clone(), Some(5))
        .await?;
    broker
        .run_api_call(metadata_request.clone(), Some(6))
        .await?;
    broker
        .run_api_call(metadata_request.clone(), Some(7))
        .await?;
    broker
        .run_api_call(metadata_request.clone(), Some(8))
        .await?;
    let metadata = broker.run_api_call(metadata_request, Some(9)).await?;
    println!("{:#?}", metadata);
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
