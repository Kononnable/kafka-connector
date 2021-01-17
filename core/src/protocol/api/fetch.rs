use super::prelude::*;

pub type FetchRequest = FetchRequest12;
pub type FetchResponse = FetchResponse12;
pub fn serialize_fetch_request(data:FetchRequest,version:i32, buf: &mut BytesMut) -> Result<(),Error> {
    match version {
        0 => ToBytes::serialize(&FetchRequest0::try_from(data)?,buf),
        1 => ToBytes::serialize(&FetchRequest1::try_from(data)?,buf),
        2 => ToBytes::serialize(&FetchRequest2::try_from(data)?,buf),
        3 => ToBytes::serialize(&FetchRequest3::try_from(data)?,buf),
        4 => ToBytes::serialize(&FetchRequest4::try_from(data)?,buf),
        5 => ToBytes::serialize(&FetchRequest5::try_from(data)?,buf),
        6 => ToBytes::serialize(&FetchRequest6::try_from(data)?,buf),
        7 => ToBytes::serialize(&FetchRequest7::try_from(data)?,buf),
        8 => ToBytes::serialize(&FetchRequest8::try_from(data)?,buf),
        9 => ToBytes::serialize(&FetchRequest9::try_from(data)?,buf),
        10 => ToBytes::serialize(&FetchRequest10::try_from(data)?,buf),
        11 => ToBytes::serialize(&FetchRequest11::try_from(data)?,buf),
        _ => ToBytes::serialize(&data,buf),
    }
    Ok(())
}
pub fn deserialize_fetch_response<T>(version:i32, buf: &mut T) -> Result<FetchResponse,Error> where T: Iterator<Item=u8> {
    Ok(match version {
        0 =>  FetchResponse0::deserialize(buf).try_into()?,
        1 =>  FetchResponse1::deserialize(buf).try_into()?,
        2 =>  FetchResponse2::deserialize(buf).try_into()?,
        3 =>  FetchResponse3::deserialize(buf).try_into()?,
        4 =>  FetchResponse4::deserialize(buf).try_into()?,
        5 =>  FetchResponse5::deserialize(buf).try_into()?,
        6 =>  FetchResponse6::deserialize(buf).try_into()?,
        7 =>  FetchResponse7::deserialize(buf).try_into()?,
        8 =>  FetchResponse8::deserialize(buf).try_into()?,
        9 =>  FetchResponse9::deserialize(buf).try_into()?,
        10 =>  FetchResponse10::deserialize(buf).try_into()?,
        11 =>  FetchResponse11::deserialize(buf).try_into()?,
        _ => FetchResponse::deserialize(buf),
    })
}

#[derive(Default,ToBytes)]
pub struct FetchRequest0 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: FetchRequestTopics0,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics0 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions0,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions0 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest1 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: FetchRequestTopics1,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics1 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions1,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions1 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest2 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: FetchRequestTopics2,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics2 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions2,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions2 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest3 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub topics: FetchRequestTopics3,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics3 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions3,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions3 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest4 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub topics: FetchRequestTopics4,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics4 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions4,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions4 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest5 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub topics: FetchRequestTopics5,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics5 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions5,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions5 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest6 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub topics: FetchRequestTopics6,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics6 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions6,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions6 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest7 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: FetchRequestTopics7,
    pub forgotten_topics_data: Optional<FetchRequestForgottenTopicsData7>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics7 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions7,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions7 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestForgottenTopicsData7 { 
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest8 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: FetchRequestTopics8,
    pub forgotten_topics_data: Optional<FetchRequestForgottenTopicsData8>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics8 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions8,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions8 { 
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestForgottenTopicsData8 { 
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest9 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: FetchRequestTopics9,
    pub forgotten_topics_data: Optional<FetchRequestForgottenTopicsData9>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics9 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions9,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions9 { 
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestForgottenTopicsData9 { 
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest10 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: FetchRequestTopics10,
    pub forgotten_topics_data: Optional<FetchRequestForgottenTopicsData10>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics10 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions10,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions10 { 
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestForgottenTopicsData10 { 
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest11 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: FetchRequestTopics11,
    pub forgotten_topics_data: Optional<FetchRequestForgottenTopicsData11>,
    pub rack_id: Optional<String>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics11 { 
    pub topic: String,
    pub partitions: FetchRequestTopicsPartitions11,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions11 { 
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestForgottenTopicsData11 { 
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequest12 { 
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: FetchRequestTopics12,
    pub forgotten_topics_data: Optional<FetchRequestForgottenTopicsData12>,
    pub rack_id: Optional<CompactString>,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopics12 { 
    pub topic: CompactString,
    pub partitions: FetchRequestTopicsPartitions12,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestTopicsPartitions12 { 
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub last_fetched_epoch: Optional<Int32>,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default,ToBytes)]
pub struct FetchRequestForgottenTopicsData12 { 
    pub topic: CompactString,
    pub partitions: Vec<Int32>,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse0 { 
    pub responses: FetchResponseResponses0,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses0 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses0,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses0 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse1 { 
    pub throttle_time_ms: Optional<Int32>,
    pub responses: FetchResponseResponses1,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses1 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses1,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses1 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse2 { 
    pub throttle_time_ms: Optional<Int32>,
    pub responses: FetchResponseResponses2,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses2 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses2,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses2 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse3 { 
    pub throttle_time_ms: Optional<Int32>,
    pub responses: FetchResponseResponses3,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses3 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses3,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses3 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse4 { 
    pub throttle_time_ms: Optional<Int32>,
    pub responses: FetchResponseResponses4,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses4 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses4,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses4 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions4>,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions4 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse5 { 
    pub throttle_time_ms: Optional<Int32>,
    pub responses: FetchResponseResponses5,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses5 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses5,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses5 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions5>,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions5 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse6 { 
    pub throttle_time_ms: Optional<Int32>,
    pub responses: FetchResponseResponses6,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses6 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses6,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses6 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions6>,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions6 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse7 { 
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: FetchResponseResponses7,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses7 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses7,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses7 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions7>,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions7 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse8 { 
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: FetchResponseResponses8,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses8 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses8,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses8 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions8>,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions8 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse9 { 
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: FetchResponseResponses9,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses9 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses9,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses9 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions9>,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions9 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse10 { 
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: FetchResponseResponses10,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses10 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses10,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses10 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions10>,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions10 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse11 { 
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: FetchResponseResponses11,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses11 { 
    pub topic: String,
    pub partition_responses: FetchResponseResponsesPartitionResponses11,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses11 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions11>,
    pub preferred_read_replica: Optional<Int32>,
    pub record_set: Records,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions11 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default,FromBytes)]
pub struct FetchResponse12 { 
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: FetchResponseResponses12,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponses12 { 
    pub topic: CompactString,
    pub partition_responses: FetchResponseResponsesPartitionResponses12,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponses12 { 
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions: Optional<FetchResponseResponsesPartitionResponsesAbortedTransactions12>,
    pub preferred_read_replica: Optional<Int32>,
    pub record_set: CompactRecords,
}

#[derive(Default,FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions12 { 
    pub producer_id: Int64,
    pub first_offset: Int64,
}

impl TryFrom<FetchRequest12> for FetchRequest0{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.max_bytes.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",0,"max_bytes"))?
        }
        if latest.isolation_level.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",0,"isolation_level"))?
        }
        if latest.session_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",0,"session_id"))?
        }
        if latest.session_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",0,"session_epoch"))?
        }
        if latest.forgotten_topics_data.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",0,"forgotten_topics_data"))?
        }
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",0,"rack_id"))?
        }
        Ok(FetchRequest0{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics0{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics0{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions0{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",0,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",0,"last_fetched_epoch"))?
        }
        if latest.log_start_offset.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",0,"log_start_offset"))?
        }
        Ok(FetchRequestTopicsPartitions0{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest1{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.max_bytes.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",1,"max_bytes"))?
        }
        if latest.isolation_level.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",1,"isolation_level"))?
        }
        if latest.session_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",1,"session_id"))?
        }
        if latest.session_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",1,"session_epoch"))?
        }
        if latest.forgotten_topics_data.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",1,"forgotten_topics_data"))?
        }
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",1,"rack_id"))?
        }
        Ok(FetchRequest1{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics1{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics1{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions1{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",1,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",1,"last_fetched_epoch"))?
        }
        if latest.log_start_offset.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",1,"log_start_offset"))?
        }
        Ok(FetchRequestTopicsPartitions1{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest2{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.max_bytes.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",2,"max_bytes"))?
        }
        if latest.isolation_level.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",2,"isolation_level"))?
        }
        if latest.session_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",2,"session_id"))?
        }
        if latest.session_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",2,"session_epoch"))?
        }
        if latest.forgotten_topics_data.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",2,"forgotten_topics_data"))?
        }
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",2,"rack_id"))?
        }
        Ok(FetchRequest2{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics2{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics2{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions2{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",2,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",2,"last_fetched_epoch"))?
        }
        if latest.log_start_offset.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",2,"log_start_offset"))?
        }
        Ok(FetchRequestTopicsPartitions2{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest3{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.isolation_level.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",3,"isolation_level"))?
        }
        if latest.session_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",3,"session_id"))?
        }
        if latest.session_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",3,"session_epoch"))?
        }
        if latest.forgotten_topics_data.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",3,"forgotten_topics_data"))?
        }
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",3,"rack_id"))?
        }
        Ok(FetchRequest3{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics3{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics3{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions3{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",3,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",3,"last_fetched_epoch"))?
        }
        if latest.log_start_offset.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",3,"log_start_offset"))?
        }
        Ok(FetchRequestTopicsPartitions3{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest4{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.session_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",4,"session_id"))?
        }
        if latest.session_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",4,"session_epoch"))?
        }
        if latest.forgotten_topics_data.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",4,"forgotten_topics_data"))?
        }
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",4,"rack_id"))?
        }
        Ok(FetchRequest4{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics4{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics4{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions4{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",4,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",4,"last_fetched_epoch"))?
        }
        if latest.log_start_offset.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",4,"log_start_offset"))?
        }
        Ok(FetchRequestTopicsPartitions4{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest5{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.session_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",5,"session_id"))?
        }
        if latest.session_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",5,"session_epoch"))?
        }
        if latest.forgotten_topics_data.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",5,"forgotten_topics_data"))?
        }
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",5,"rack_id"))?
        }
        Ok(FetchRequest5{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics5{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics5{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions5{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",5,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",5,"last_fetched_epoch"))?
        }
        Ok(FetchRequestTopicsPartitions5{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest6{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.session_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",6,"session_id"))?
        }
        if latest.session_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",6,"session_epoch"))?
        }
        if latest.forgotten_topics_data.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",6,"forgotten_topics_data"))?
        }
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",6,"rack_id"))?
        }
        Ok(FetchRequest6{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics6{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics6{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions6{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",6,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",6,"last_fetched_epoch"))?
        }
        Ok(FetchRequestTopicsPartitions6{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest7{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",7,"rack_id"))?
        }
        Ok(FetchRequest7{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.try_into()?,
            forgotten_topics_data: latest.forgotten_topics_data.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics7{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics7{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions7{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",7,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",7,"last_fetched_epoch"))?
        }
        Ok(FetchRequestTopicsPartitions7{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData7{
    type Error = Error;
    fn try_from(latest:FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData7{
            topic: latest.topic,
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest8{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",8,"rack_id"))?
        }
        Ok(FetchRequest8{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.try_into()?,
            forgotten_topics_data: latest.forgotten_topics_data.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics8{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics8{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions8{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",8,"current_leader_epoch"))?
        }
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",8,"last_fetched_epoch"))?
        }
        Ok(FetchRequestTopicsPartitions8{
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData8{
    type Error = Error;
    fn try_from(latest:FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData8{
            topic: latest.topic,
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest9{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",9,"rack_id"))?
        }
        Ok(FetchRequest9{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.try_into()?,
            forgotten_topics_data: latest.forgotten_topics_data.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics9{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics9{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions9{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",9,"last_fetched_epoch"))?
        }
        Ok(FetchRequestTopicsPartitions9{
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData9{
    type Error = Error;
    fn try_from(latest:FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData9{
            topic: latest.topic,
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest10{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        if latest.rack_id.is_some() {
            Err(Error::OldKafkaVersion("FetchRequest",10,"rack_id"))?
        }
        Ok(FetchRequest10{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.try_into()?,
            forgotten_topics_data: latest.forgotten_topics_data.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics10{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics10{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions10{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",10,"last_fetched_epoch"))?
        }
        Ok(FetchRequestTopicsPartitions10{
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData10{
    type Error = Error;
    fn try_from(latest:FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData10{
            topic: latest.topic,
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest11{
    type Error = Error;
    fn try_from(latest:FetchRequest12) -> Result<Self, Self::Error> {
        Ok(FetchRequest11{
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.try_into()?,
            forgotten_topics_data: latest.forgotten_topics_data.try_into()?,
            rack_id: latest.rack_id,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics11{
    type Error = Error;
    fn try_from(latest:FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics11{
            topic: latest.topic,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions11{
    type Error = Error;
    fn try_from(latest:FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.last_fetched_epoch.is_some() {
            Err(Error::OldKafkaVersion("FetchRequestTopicsPartitions",11,"last_fetched_epoch"))?
        }
        Ok(FetchRequestTopicsPartitions11{
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData11{
    type Error = Error;
    fn try_from(latest:FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData11{
            topic: latest.topic,
            partitions: latest.partitions,
        })
    }
}


impl TryFrom<FetchResponse0> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse0) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses0> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses0) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses0> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses0) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponse1> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse1) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses1> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses1) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses1> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses1) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponse2> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse2) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses2> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses2) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses2> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses2) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponse3> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse3) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses3> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses3) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses3> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses3) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponse4> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse4) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses4> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses4) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses4> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses4) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            aborted_transactions: older.aborted_transactions.try_into()?,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponsesAbortedTransactions4> for FetchResponseResponsesPartitionResponsesAbortedTransactions12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponsesAbortedTransactions4) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponsesAbortedTransactions12{
            producer_id: older.producer_id,
            first_offset: older.first_offset,
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        })
    }
}

impl TryFrom<FetchResponse5> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse5) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses5> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses5) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses5> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses5) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older.aborted_transactions.try_into()?,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponsesAbortedTransactions5> for FetchResponseResponsesPartitionResponsesAbortedTransactions12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponsesAbortedTransactions5) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponsesAbortedTransactions12{
            producer_id: older.producer_id,
            first_offset: older.first_offset,
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        })
    }
}

impl TryFrom<FetchResponse6> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse6) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses6> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses6) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses6> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses6) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older.aborted_transactions.try_into()?,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponsesAbortedTransactions6> for FetchResponseResponsesPartitionResponsesAbortedTransactions12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponsesAbortedTransactions6) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponsesAbortedTransactions12{
            producer_id: older.producer_id,
            first_offset: older.first_offset,
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        })
    }
}

impl TryFrom<FetchResponse7> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse7) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses7> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses7) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses7> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses7) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older.aborted_transactions.try_into()?,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponsesAbortedTransactions7> for FetchResponseResponsesPartitionResponsesAbortedTransactions12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponsesAbortedTransactions7) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponsesAbortedTransactions12{
            producer_id: older.producer_id,
            first_offset: older.first_offset,
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        })
    }
}

impl TryFrom<FetchResponse8> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse8) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses8> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses8) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses8> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses8) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older.aborted_transactions.try_into()?,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponsesAbortedTransactions8> for FetchResponseResponsesPartitionResponsesAbortedTransactions12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponsesAbortedTransactions8) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponsesAbortedTransactions12{
            producer_id: older.producer_id,
            first_offset: older.first_offset,
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        })
    }
}

impl TryFrom<FetchResponse9> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse9) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses9> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses9) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses9> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses9) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older.aborted_transactions.try_into()?,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponsesAbortedTransactions9> for FetchResponseResponsesPartitionResponsesAbortedTransactions12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponsesAbortedTransactions9) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponsesAbortedTransactions12{
            producer_id: older.producer_id,
            first_offset: older.first_offset,
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        })
    }
}

impl TryFrom<FetchResponse10> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse10) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses10> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses10) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses10> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses10) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older.aborted_transactions.try_into()?,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponsesAbortedTransactions10> for FetchResponseResponsesPartitionResponsesAbortedTransactions12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponsesAbortedTransactions10) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponsesAbortedTransactions12{
            producer_id: older.producer_id,
            first_offset: older.first_offset,
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        })
    }
}

impl TryFrom<FetchResponse11> for FetchResponse12{
    type Error = Error;
    fn try_from(older:FetchResponse11) -> Result<Self, Self::Error> {
        Ok(FetchResponse12{
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.try_into()?,
            ..FetchResponse12::default()
        })
    }
}

impl TryFrom<FetchResponseResponses11> for FetchResponseResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponses11) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponses12{
            topic: older.topic,
            partition_responses: older.partition_responses.try_into()?,
            ..FetchResponseResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponses11> for FetchResponseResponsesPartitionResponses12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponses11) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponses12{
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older.aborted_transactions.try_into()?,
            preferred_read_replica: older.preferred_read_replica,
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
        })
    }
}

impl TryFrom<FetchResponseResponsesPartitionResponsesAbortedTransactions11> for FetchResponseResponsesPartitionResponsesAbortedTransactions12{
    type Error = Error;
    fn try_from(older:FetchResponseResponsesPartitionResponsesAbortedTransactions11) -> Result<Self, Self::Error> {
        Ok(FetchResponseResponsesPartitionResponsesAbortedTransactions12{
            producer_id: older.producer_id,
            first_offset: older.first_offset,
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        })
    }
}

