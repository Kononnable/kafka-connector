use super::super::to_bytes::ToBytes;
use kafka_connector_derive::ToBytes;

#[derive(Debug, ToBytes)]
pub struct FetchRequest0 {
    pub replica_id: i32,
    pub max_wait_ms: i32,
    pub min_bytes: i32,
    pub topics: Vec<Topic0>
}

#[derive(Debug, ToBytes)]
pub struct Topic0 {
    pub topic: String,
    pub partitions: Vec<Partition0>
}

#[derive(Debug, ToBytes)]
pub struct Partition0 {
    pub partition: i32,
    pub fetch_offset: i64,
    pub partition_max_bytes: i32,
}

pub type FetchRequest1 = FetchRequest0;
pub type FetchRequest2 = FetchRequest0;

#[derive(Debug, ToBytes)]
pub struct FetchRequest3 {
    pub replica_id: i32,
    pub max_wait_ms: i32,
    pub min_bytes: i32,
    pub max_bytes: i32,
    pub topics: Vec<Topic0>
}

#[derive(Debug, ToBytes)]
pub struct FetchRequest4 {
    pub replica_id: i32,
    pub max_wait_ms: i32,
    pub min_bytes: i32,
    pub max_bytes: i32,
    pub isolation_level: i8,
    pub topics: Vec<Topic0>
}


#[derive(Debug, ToBytes)]
pub struct FetchRequest5 {
    pub replica_id: i32,
    pub max_wait_ms: i32,
    pub min_bytes: i32,
    pub max_bytes: i32,
    pub isolation_level: i8,
    pub topics: Vec<Topic5>
}
#[derive(Debug, ToBytes)]
pub struct Topic5 {
    pub topic: String,
    pub partitions: Vec<Partition5>
}

#[derive(Debug, ToBytes)]
pub struct Partition5 {
    pub partition: i32,
    pub fetch_offset: i64,
    pub log_start_offset: i64,
    pub partition_max_bytes: i32,
}

pub type FetchRequest6 = FetchRequest5;


#[derive(Debug, ToBytes)]
pub struct FetchRequest7 {
    pub replica_id: i32,
    pub max_wait_ms: i32,
    pub min_bytes: i32,
    pub max_bytes: i32,
    pub isolation_level: i8,
    pub session_id: i32,
    pub session_epoch: i32,
    pub topics: Vec<Topic7>,
    pub forgotten_topics_data: Vec<ForgottenTopic7>,
}
#[derive(Debug, ToBytes)]
pub struct Topic7 {
    pub topic: String,
    pub partitions: Vec<Partition7>
}

#[derive(Debug, ToBytes)]
pub struct Partition7 {
    pub partition: i32,
    pub fetch_offset: i64,
    pub log_start_offset: i64,
    pub partition_max_bytes: i32,
}
#[derive(Debug, ToBytes)]
pub struct ForgottenTopic7 {
    pub topic: String,
    pub partitions: Vec<i32>
}
pub type FetchRequest8 = FetchRequest7;
