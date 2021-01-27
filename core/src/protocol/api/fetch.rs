use super::prelude::*;

pub type FetchRequest = FetchRequest12;
pub type FetchResponse = FetchResponse12;
pub fn serialize_fetch_request(
    data: FetchRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&FetchRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&FetchRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&FetchRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&FetchRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&FetchRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&FetchRequest5::try_from(data)?, buf),
        6 => ToBytes::serialize(&FetchRequest6::try_from(data)?, buf),
        7 => ToBytes::serialize(&FetchRequest7::try_from(data)?, buf),
        8 => ToBytes::serialize(&FetchRequest8::try_from(data)?, buf),
        9 => ToBytes::serialize(&FetchRequest9::try_from(data)?, buf),
        10 => ToBytes::serialize(&FetchRequest10::try_from(data)?, buf),
        11 => ToBytes::serialize(&FetchRequest11::try_from(data)?, buf),
        12 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_fetch_response(version: i32, buf: &mut Bytes) -> FetchResponse {
    match version {
        0 => FetchResponse0::deserialize(buf).into(),
        1 => FetchResponse1::deserialize(buf).into(),
        2 => FetchResponse2::deserialize(buf).into(),
        3 => FetchResponse3::deserialize(buf).into(),
        4 => FetchResponse4::deserialize(buf).into(),
        5 => FetchResponse5::deserialize(buf).into(),
        6 => FetchResponse6::deserialize(buf).into(),
        7 => FetchResponse7::deserialize(buf).into(),
        8 => FetchResponse8::deserialize(buf).into(),
        9 => FetchResponse9::deserialize(buf).into(),
        10 => FetchResponse10::deserialize(buf).into(),
        11 => FetchResponse11::deserialize(buf).into(),
        12 => FetchResponse::deserialize(buf),
        _ => FetchResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest0 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: Vec<FetchRequestTopics0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics0 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions0 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest1 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: Vec<FetchRequestTopics1>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics1 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions1>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions1 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest2 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: Vec<FetchRequestTopics2>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics2 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions2>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions2 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest3 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics3>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics3 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions3>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions3 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest4 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub topics: Vec<FetchRequestTopics4>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics4 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions4>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions4 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest5 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub topics: Vec<FetchRequestTopics5>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics5 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions5>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions5 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest6 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub topics: Vec<FetchRequestTopics6>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics6 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions6>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions6 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest7 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics7>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData7>>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics7 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions7>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions7 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestForgottenTopicsData7 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest8 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics8>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData8>>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics8 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions8>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions8 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestForgottenTopicsData8 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest9 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics9>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData9>>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics9 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions9>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions9 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestForgottenTopicsData9 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest10 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics10>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData10>>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics10 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions10>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions10 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestForgottenTopicsData10 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest11 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics11>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData11>>,
    pub rack_id: Optional<String>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics11 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions11>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions11 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestForgottenTopicsData11 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequest12 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics12>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData12>>,
    pub rack_id: Optional<CompactString>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopics12 {
    pub topic: CompactString,
    pub partitions: Vec<FetchRequestTopicsPartitions12>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestTopicsPartitions12 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub last_fetched_epoch: Optional<Int32>,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct FetchRequestForgottenTopicsData12 {
    pub topic: CompactString,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse0 {
    pub responses: Vec<FetchResponseResponses0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses0 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses0 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses1 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses1 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses2>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses2 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses2>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses2 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses3>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses3 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses3>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses3 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses4>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses4 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses4>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses4 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions4>>,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions4 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses5>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses5 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses5>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses5 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions5>>,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions5 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse6 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses6>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses6 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses6>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses6 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions6>>,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions6 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse7 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses7>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses7 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses7>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses7 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions7>>,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions7 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse8 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses8>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses8 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses8>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses8 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions8>>,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions8 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse9 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses9>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses9 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses9>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses9 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions9>>,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions9 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse10 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses10>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses10 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses10>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses10 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions10>>,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions10 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse11 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses11>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses11 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses11>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses11 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions11>>,
    pub preferred_read_replica: Optional<Int32>,
    pub record_set: Records,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions11 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponse12 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses12>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponses12 {
    pub topic: CompactString,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses12>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses12 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions12>>,
    pub preferred_read_replica: Optional<Int32>,
    pub record_set: CompactRecords,
}

#[derive(Default, Debug, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

impl TryFrom<FetchRequest12> for FetchRequest0 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.max_bytes.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 0, "max_bytes"));
        }
        if latest.isolation_level.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 0, "isolation_level"));
        }
        if latest.session_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 0, "session_id"));
        }
        if latest.session_epoch.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 0, "session_epoch"));
        }
        if latest.forgotten_topics_data.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequest",
                0,
                "forgotten_topics_data",
            ));
        }
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 0, "rack_id"));
        }
        Ok(FetchRequest0 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics0 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics0 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions0 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                0,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                0,
                "last_fetched_epoch",
            ));
        }
        if latest.log_start_offset.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                0,
                "log_start_offset",
            ));
        }
        Ok(FetchRequestTopicsPartitions0 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest1 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.max_bytes.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 1, "max_bytes"));
        }
        if latest.isolation_level.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 1, "isolation_level"));
        }
        if latest.session_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 1, "session_id"));
        }
        if latest.session_epoch.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 1, "session_epoch"));
        }
        if latest.forgotten_topics_data.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequest",
                1,
                "forgotten_topics_data",
            ));
        }
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 1, "rack_id"));
        }
        Ok(FetchRequest1 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics1 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics1 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions1 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                1,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                1,
                "last_fetched_epoch",
            ));
        }
        if latest.log_start_offset.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                1,
                "log_start_offset",
            ));
        }
        Ok(FetchRequestTopicsPartitions1 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest2 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.max_bytes.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 2, "max_bytes"));
        }
        if latest.isolation_level.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 2, "isolation_level"));
        }
        if latest.session_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 2, "session_id"));
        }
        if latest.session_epoch.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 2, "session_epoch"));
        }
        if latest.forgotten_topics_data.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequest",
                2,
                "forgotten_topics_data",
            ));
        }
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 2, "rack_id"));
        }
        Ok(FetchRequest2 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics2 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics2 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions2 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                2,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                2,
                "last_fetched_epoch",
            ));
        }
        if latest.log_start_offset.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                2,
                "log_start_offset",
            ));
        }
        Ok(FetchRequestTopicsPartitions2 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest3 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.isolation_level.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 3, "isolation_level"));
        }
        if latest.session_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 3, "session_id"));
        }
        if latest.session_epoch.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 3, "session_epoch"));
        }
        if latest.forgotten_topics_data.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequest",
                3,
                "forgotten_topics_data",
            ));
        }
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 3, "rack_id"));
        }
        Ok(FetchRequest3 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics3 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics3 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions3 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                3,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                3,
                "last_fetched_epoch",
            ));
        }
        if latest.log_start_offset.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                3,
                "log_start_offset",
            ));
        }
        Ok(FetchRequestTopicsPartitions3 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest4 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.session_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 4, "session_id"));
        }
        if latest.session_epoch.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 4, "session_epoch"));
        }
        if latest.forgotten_topics_data.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequest",
                4,
                "forgotten_topics_data",
            ));
        }
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 4, "rack_id"));
        }
        Ok(FetchRequest4 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics4 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics4 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions4 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                4,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                4,
                "last_fetched_epoch",
            ));
        }
        if latest.log_start_offset.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                4,
                "log_start_offset",
            ));
        }
        Ok(FetchRequestTopicsPartitions4 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest5 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.session_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 5, "session_id"));
        }
        if latest.session_epoch.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 5, "session_epoch"));
        }
        if latest.forgotten_topics_data.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequest",
                5,
                "forgotten_topics_data",
            ));
        }
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 5, "rack_id"));
        }
        Ok(FetchRequest5 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics5 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics5 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions5 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                5,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                5,
                "last_fetched_epoch",
            ));
        }
        Ok(FetchRequestTopicsPartitions5 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest6 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.session_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 6, "session_id"));
        }
        if latest.session_epoch.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 6, "session_epoch"));
        }
        if latest.forgotten_topics_data.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequest",
                6,
                "forgotten_topics_data",
            ));
        }
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 6, "rack_id"));
        }
        Ok(FetchRequest6 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics6 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics6 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions6 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                6,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                6,
                "last_fetched_epoch",
            ));
        }
        Ok(FetchRequestTopicsPartitions6 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest7 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 7, "rack_id"));
        }
        Ok(FetchRequest7 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            forgotten_topics_data: latest
                .forgotten_topics_data
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics7 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics7 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions7 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                7,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                7,
                "last_fetched_epoch",
            ));
        }
        Ok(FetchRequestTopicsPartitions7 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData7 {
    type Error = Error;
    fn try_from(latest: FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData7 {
            topic: latest.topic.into(),
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest8 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 8, "rack_id"));
        }
        Ok(FetchRequest8 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            forgotten_topics_data: latest
                .forgotten_topics_data
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics8 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics8 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions8 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                8,
                "current_leader_epoch",
            ));
        }
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                8,
                "last_fetched_epoch",
            ));
        }
        Ok(FetchRequestTopicsPartitions8 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData8 {
    type Error = Error;
    fn try_from(latest: FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData8 {
            topic: latest.topic.into(),
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest9 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 9, "rack_id"));
        }
        Ok(FetchRequest9 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            forgotten_topics_data: latest
                .forgotten_topics_data
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics9 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics9 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions9 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                9,
                "last_fetched_epoch",
            ));
        }
        Ok(FetchRequestTopicsPartitions9 {
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData9 {
    type Error = Error;
    fn try_from(latest: FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData9 {
            topic: latest.topic.into(),
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest10 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.rack_id.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 10, "rack_id"));
        }
        Ok(FetchRequest10 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            forgotten_topics_data: latest
                .forgotten_topics_data
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics10 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics10 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions10 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                10,
                "last_fetched_epoch",
            ));
        }
        Ok(FetchRequestTopicsPartitions10 {
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData10 {
    type Error = Error;
    fn try_from(latest: FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData10 {
            topic: latest.topic.into(),
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest11 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        Ok(FetchRequest11 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            forgotten_topics_data: latest
                .forgotten_topics_data
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
            rack_id: latest.rack_id.map(|val| val.into()),
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics11 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        Ok(FetchRequestTopics11 {
            topic: latest.topic.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions11 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopicsPartitions12) -> Result<Self, Self::Error> {
        if latest.last_fetched_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                11,
                "last_fetched_epoch",
            ));
        }
        Ok(FetchRequestTopicsPartitions11 {
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        })
    }
}

impl TryFrom<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData11 {
    type Error = Error;
    fn try_from(latest: FetchRequestForgottenTopicsData12) -> Result<Self, Self::Error> {
        Ok(FetchRequestForgottenTopicsData11 {
            topic: latest.topic.into(),
            partitions: latest.partitions,
        })
    }
}

impl From<FetchResponse0> for FetchResponse12 {
    fn from(older: FetchResponse0) -> Self {
        FetchResponse12 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses0> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses0) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses0>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses0) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponse1> for FetchResponse12 {
    fn from(older: FetchResponse1) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses1> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses1) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses1>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses1) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponse2> for FetchResponse12 {
    fn from(older: FetchResponse2) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses2> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses2) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses2>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses2) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponse3> for FetchResponse12 {
    fn from(older: FetchResponse3) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses3> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses3) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses3>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses3) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponse4> for FetchResponse12 {
    fn from(older: FetchResponse4) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses4> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses4) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses4>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses4) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            aborted_transactions: older
                .aborted_transactions
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponseResponsesPartitionResponsesAbortedTransactions4>
    for FetchResponseResponsesPartitionResponsesAbortedTransactions12
{
    fn from(older: FetchResponseResponsesPartitionResponsesAbortedTransactions4) -> Self {
        FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
            producer_id: older.producer_id,
            first_offset: older.first_offset,
        }
    }
}

impl From<FetchResponse5> for FetchResponse12 {
    fn from(older: FetchResponse5) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses5> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses5) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses5>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses5) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older
                .aborted_transactions
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponseResponsesPartitionResponsesAbortedTransactions5>
    for FetchResponseResponsesPartitionResponsesAbortedTransactions12
{
    fn from(older: FetchResponseResponsesPartitionResponsesAbortedTransactions5) -> Self {
        FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
            producer_id: older.producer_id,
            first_offset: older.first_offset,
        }
    }
}

impl From<FetchResponse6> for FetchResponse12 {
    fn from(older: FetchResponse6) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses6> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses6) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses6>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses6) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older
                .aborted_transactions
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponseResponsesPartitionResponsesAbortedTransactions6>
    for FetchResponseResponsesPartitionResponsesAbortedTransactions12
{
    fn from(older: FetchResponseResponsesPartitionResponsesAbortedTransactions6) -> Self {
        FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
            producer_id: older.producer_id,
            first_offset: older.first_offset,
        }
    }
}

impl From<FetchResponse7> for FetchResponse12 {
    fn from(older: FetchResponse7) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<FetchResponseResponses7> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses7) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses7>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses7) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older
                .aborted_transactions
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponseResponsesPartitionResponsesAbortedTransactions7>
    for FetchResponseResponsesPartitionResponsesAbortedTransactions12
{
    fn from(older: FetchResponseResponsesPartitionResponsesAbortedTransactions7) -> Self {
        FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
            producer_id: older.producer_id,
            first_offset: older.first_offset,
        }
    }
}

impl From<FetchResponse8> for FetchResponse12 {
    fn from(older: FetchResponse8) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<FetchResponseResponses8> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses8) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses8>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses8) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older
                .aborted_transactions
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponseResponsesPartitionResponsesAbortedTransactions8>
    for FetchResponseResponsesPartitionResponsesAbortedTransactions12
{
    fn from(older: FetchResponseResponsesPartitionResponsesAbortedTransactions8) -> Self {
        FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
            producer_id: older.producer_id,
            first_offset: older.first_offset,
        }
    }
}

impl From<FetchResponse9> for FetchResponse12 {
    fn from(older: FetchResponse9) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<FetchResponseResponses9> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses9) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses9>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses9) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older
                .aborted_transactions
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponseResponsesPartitionResponsesAbortedTransactions9>
    for FetchResponseResponsesPartitionResponsesAbortedTransactions12
{
    fn from(older: FetchResponseResponsesPartitionResponsesAbortedTransactions9) -> Self {
        FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
            producer_id: older.producer_id,
            first_offset: older.first_offset,
        }
    }
}

impl From<FetchResponse10> for FetchResponse12 {
    fn from(older: FetchResponse10) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<FetchResponseResponses10> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses10) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses10>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses10) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older
                .aborted_transactions
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            record_set: older.record_set.into(),
            ..FetchResponseResponsesPartitionResponses12::default()
        }
    }
}

impl From<FetchResponseResponsesPartitionResponsesAbortedTransactions10>
    for FetchResponseResponsesPartitionResponsesAbortedTransactions12
{
    fn from(older: FetchResponseResponsesPartitionResponsesAbortedTransactions10) -> Self {
        FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
            producer_id: older.producer_id,
            first_offset: older.first_offset,
        }
    }
}

impl From<FetchResponse11> for FetchResponse12 {
    fn from(older: FetchResponse11) -> Self {
        FetchResponse12 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            session_id: older.session_id,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<FetchResponseResponses11> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses11) -> Self {
        FetchResponseResponses12 {
            topic: older.topic.into(),
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponses11>
    for FetchResponseResponsesPartitionResponses12
{
    fn from(older: FetchResponseResponsesPartitionResponses11) -> Self {
        FetchResponseResponsesPartitionResponses12 {
            partition: older.partition,
            error_code: older.error_code,
            high_watermark: older.high_watermark,
            last_stable_offset: older.last_stable_offset,
            log_start_offset: older.log_start_offset,
            aborted_transactions: older
                .aborted_transactions
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            preferred_read_replica: older.preferred_read_replica,
            record_set: older.record_set.into(),
        }
    }
}

impl From<FetchResponseResponsesPartitionResponsesAbortedTransactions11>
    for FetchResponseResponsesPartitionResponsesAbortedTransactions12
{
    fn from(older: FetchResponseResponsesPartitionResponsesAbortedTransactions11) -> Self {
        FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
            producer_id: older.producer_id,
            first_offset: older.first_offset,
        }
    }
}
