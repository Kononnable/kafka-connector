use kafka_connector::kafka_client::BrokerClient;

use kafka_connector::protocol;
use protocol::{
    api::{
        fetch::{FetchRequestTopics12, FetchRequestTopicsPartitions12},
        metadata::MetadataRequestTopics9,
        ApiNumbers,
    },
    custom_types::{optional::Optional, tag_buffer::TagBuffer},
};

const BROKER: &str = "127.0.0.1:9092";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut broker = BrokerClient::new(BROKER, "kafka-connector-test".to_owned()).await?;

    let metadata_request = protocol::api::metadata::MetadataRequest {
        topics: vec![MetadataRequestTopics9 {
            name: "test2".to_owned(),
            tag_buffer: Optional::Some(TagBuffer {}),
        }],
        allow_auto_topic_creation: Optional::None,
        include_cluster_authorized_operations: Optional::None,
        include_topic_authorized_operations: Optional::None,
        tag_buffer: Optional::Some(TagBuffer {}),
    };
    let supported_version = broker
        .supported_versions
        .get(&(ApiNumbers::Metadata as i16));
    println!("supported_versions {:?}", supported_version);
    let metadata = broker.run_api_call(metadata_request, Some(9)).await?;
    println!("{:#?}", metadata);

    let partition = FetchRequestTopicsPartitions12 {
        partition: 0,
        current_leader_epoch: Optional::None,
        fetch_offset: 0,
        last_fetched_epoch: Optional::None,
        log_start_offset: Optional::None,
        partition_max_bytes: 100000,
        tag_buffer: Optional::None,
    };

    let fetch_request = protocol::api::fetch::FetchRequest {
        forgotten_topics_data: Optional::None,
        isolation_level: Optional::Some(1),
        replica_id: -1,
        max_wait_ms: 10000,
        min_bytes: 100,
        max_bytes: Optional::Some(3200000),
        session_id: Optional::None,
        session_epoch: Optional::None,
        topics: vec![FetchRequestTopics12 {
            topic: "test3".to_owned(),
            tag_buffer: Optional::Some(TagBuffer {}),
            partitions: vec![partition],
        }],
        rack_id: Optional::Some("".to_owned()),
        tag_buffer: Optional::None,
    };
    println!("{:#?}", fetch_request);

    let supported_version = broker.supported_versions.get(&(ApiNumbers::Fetch as i16));
    println!("supported_versions {:?}", supported_version);
    let fetch = broker.run_api_call(fetch_request, Some(4)).await?;
    println!("{:#?}", fetch);

    Ok(())
}
