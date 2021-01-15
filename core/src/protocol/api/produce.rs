use super::super::from_bytes::FromBytes;
use super::super::to_bytes::ToBytes;
use kafka_connector_derive::FromBytes;
use kafka_connector_derive::ToBytes;
use crate::protocol::types::Records;

#[derive(Debug, ToBytes)]
pub struct ProduceRequest0 {
    pub acks: i16,
    pub timeout: i32,
    pub topic_data: Vec<TopicData0>
}

#[derive(Debug, ToBytes)]
pub struct TopicData0 {
    pub topic: String,
    pub data: Vec<Data0>,
}

#[derive(Debug, ToBytes)]
pub struct Data0 {
    pub partition: i32,
    pub record_set: Records,
}

pub type ProduceRequest1 = ProduceRequest0;
pub type ProduceRequest2 = ProduceRequest0;

#[derive(Debug, ToBytes)]
pub struct ProduceRequest3 {
    pub transactional_id: Option<String>,
    pub acks: i16,
    pub timeout: i32,
    pub topic_data: Vec<TopicData3>
}

#[derive(Debug, ToBytes)]
pub struct TopicData3 {
    pub topic: String,
    pub data: Vec<Data3>,
}

#[derive(Debug, ToBytes)]
pub struct Data3 {
    pub partition: i32,
    pub record_set: Records,
}

pub type ProduceRequest4 = ProduceRequest3;
pub type ProduceRequest5 = ProduceRequest3;
pub type ProduceRequest6 = ProduceRequest3;
pub type ProduceRequest7 = ProduceRequest3;
pub type ProduceRequest8 = ProduceRequest3;


#[derive(Debug, FromBytes)]
pub struct ProduceResponse0 {
    pub responses: Vec<Responses0>
}
#[derive(Debug, FromBytes)]
pub struct Responses0 {
    pub topic: String,
    pub partition_responses: Vec<PartitionResponses0>
}
#[derive(Debug, FromBytes)]
pub struct PartitionResponses0 {
    pub partition: i32,
    pub error_code: i16,
    pub base_offset: i64
}


#[derive(Debug, FromBytes)]
pub struct ProduceResponse1 {
    pub responses: Vec<Responses0>,
    pub throttle_time_ms: i32
}

#[derive(Debug, FromBytes)]
pub struct ProduceResponse2 {
    pub responses: Vec<Responses2>,
    pub throttle_time_ms: i32
}
#[derive(Debug, FromBytes)]
pub struct Responses2 {
    pub topic: String,
    pub partition_responses: Vec<PartitionResponses2>
}
#[derive(Debug, FromBytes)]
pub struct PartitionResponses2 {
    pub partition: i32,
    pub error_code: i16,
    pub base_offset: i64,
    pub log_append_time: i64,
}

pub type ProduceResponse3 = ProduceResponse2;
pub type ProduceResponse4 = ProduceResponse2;

#[derive(Debug, FromBytes)]
pub struct ProduceResponse5 {
    pub responses: Vec<Responses5>,
    pub throttle_time_ms: i32
}
#[derive(Debug, FromBytes)]
pub struct Responses5 {
    pub topic: String,
    pub partition_responses: Vec<PartitionResponses5>
}
#[derive(Debug, FromBytes)]
pub struct PartitionResponses5 {
    pub partition: i32,
    pub error_code: i16,
    pub base_offset: i64,
    pub log_append_time: i64,
    pub log_start_offset: i64,
}

pub type ProduceResponse6 = ProduceResponse2;
pub type ProduceResponse7 = ProduceResponse2;

#[derive(Debug, FromBytes)]
pub struct ProduceResponse8 {
    pub responses: Vec<Responses8>,
    pub throttle_time_ms: i32
}
#[derive(Debug, FromBytes)]
pub struct Responses8 {
    pub topic: String,
    pub partition_responses: Vec<PartitionResponses8>
}
#[derive(Debug, FromBytes)]
pub struct PartitionResponses8 {
    pub partition: i32,
    pub error_code: i16,
    pub base_offset: i64,
    pub log_append_time: i64,
    pub log_start_offset: i64,
    pub record_errors: Vec<RecordErrors8>,
    pub error_message: Option<String>,
}
#[derive(Debug, FromBytes)]
pub struct RecordErrors8 {
    pub batch_index: i32,
    pub batch_index_error_message: Option<String>
}