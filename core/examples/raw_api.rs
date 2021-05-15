use std::{net::ToSocketAddrs, sync::Arc, vec};

use kafka_connector::{
    broker::{options::KafkaClientOptions, Broker},
    protocol,
};
use protocol::{
    api::{
        fetch::{FetchRequestTopics0, FetchRequestTopicsPartitions0},
        metadata::MetadataRequestTopics0,
        produce::{ProduceRequestTopicData0, ProduceRequestTopicDataData0},
    },
    custom_types::{
        nullable_string::NullableString,
        record_batch::{
            batch::RecordBatch, record::Record, record_batch_with_size::RecordBatchWithSize,
        },
        tag_buffer::TagBuffer,
        zig_zag_vec::ZigZagVec,
    },
};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let broker = "127.0.0.1:9092".to_socket_addrs().unwrap().next().unwrap();
    let options = Arc::new(
        KafkaClientOptions::builder()
            .client_id("kafka-connector-test".to_owned())
            .build(),
    );

    let mut broker = Broker::new_wait(broker, options).await?;

    let metadata_request = protocol::api::metadata::MetadataRequest {
        topics: vec![MetadataRequestTopics0 {
            name: "test2".to_owned(),
            tag_buffer: TagBuffer {},
        }],
        allow_auto_topic_creation: false,
        include_cluster_authorized_operations: false,
        include_topic_authorized_operations: false,
        tag_buffer: TagBuffer {},
    };
    // let supported_version = broker
    //     .supported_versions
    //     .get(&(ApiNumbers::Metadata as u16));
    // println!("supported_versions {:?}", supported_version);
    let metadata = broker.run_api_call(&metadata_request, Some(9)).await?;
    println!("{:#?}", metadata);

    let partition = FetchRequestTopicsPartitions0 {
        partition: 0,
        current_leader_epoch: 0,
        fetch_offset: 0,
        last_fetched_epoch: 0,
        log_start_offset: 0,
        partition_max_bytes: 100000,
        tag_buffer: TagBuffer {},
    };

    let fetch_request = protocol::api::fetch::FetchRequest {
        forgotten_topics_data: vec![],
        isolation_level: 1,
        replica_id: -1,
        max_wait_ms: 10000,
        min_bytes: 100,
        max_bytes: 3200000,
        session_id: 0,
        session_epoch: 0,
        topics: vec![FetchRequestTopics0 {
            topic: "test3".to_owned(),
            tag_buffer: TagBuffer {},
            partitions: vec![partition],
        }],
        rack_id: "".to_owned(),
        tag_buffer: TagBuffer {},
    };
    println!("{:#?}", fetch_request);

    // let supported_version = broker.supported_versions.get(&(ApiNumbers::Fetch as u16));
    // println!("supported_versions {:?}", supported_version);
    let fetch = broker.run_api_call(&fetch_request, Some(4)).await?;
    println!("{:#?}", fetch);

    let records = RecordBatchWithSize {
        batches: vec![RecordBatch {
            attributes: 0,
            base_sequence: -1,
            partition_leader_epoch: 0,
            producer_epoch: -1,
            producer_id: -1,
            records: vec![Record {
                attributes: 0,
                headers: ZigZagVec { value: vec![] },
                key: ZigZagVec {
                    value: b"test".to_vec(),
                },
                offset: 0,
                timestamp: 1612286095600,
                value: ZigZagVec {
                    value: b"test".to_vec(),
                },
            }],
        }],
    };
    let produce_request = protocol::api::produce::ProduceRequest {
        transactional_id: NullableString::None,
        acks: 1_i16,
        timeout: 10_000,
        topic_data: vec![ProduceRequestTopicData0 {
            topic: "test3".to_owned(),
            data: vec![ProduceRequestTopicDataData0 {
                partition: 0,
                record_set: records,
            }],
        }],
    };
    // let supported_version = broker.supported_versions.get(&(ApiNumbers::Produce as u16));
    // println!("supported_versions {:?}", supported_version);
    let produce = broker.run_api_call(&produce_request, Some(4)).await?;
    println!("{:#?}", produce);

    Ok(())
}
