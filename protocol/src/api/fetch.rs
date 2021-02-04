use super::prelude::*;

pub type FetchRequest = FetchRequest12;
pub type FetchResponse = FetchResponse12;
impl ApiCall for FetchRequest {
    type Response = FetchResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        12
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::Fetch
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => false,
            6 => false,
            7 => false,
            8 => false,
            9 => false,
            10 => false,
            11 => false,
            12 => true,
            _ => true,
        }
    }
    fn serialize(
        self,
        version: i16,
        buf: &mut BytesMut,
        correlation_id: i32,
        client_id: &str,
    ) -> Result<(), Error> {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                FetchRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                FetchRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &FetchRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &FetchRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &FetchRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &FetchRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &FetchRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &FetchRequest5::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(
                &FetchRequest6::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            7 => ToBytes::serialize(
                &FetchRequest7::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            8 => ToBytes::serialize(
                &FetchRequest8::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            9 => ToBytes::serialize(
                &FetchRequest9::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            10 => ToBytes::serialize(
                &FetchRequest10::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            11 => ToBytes::serialize(
                &FetchRequest11::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            12 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, FetchResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => FetchResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => FetchResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => FetchResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => FetchResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => FetchResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => FetchResponse5::deserialize(buf, Self::is_flexible_version(version)).into(),
            6 => FetchResponse6::deserialize(buf, Self::is_flexible_version(version)).into(),
            7 => FetchResponse7::deserialize(buf, Self::is_flexible_version(version)).into(),
            8 => FetchResponse8::deserialize(buf, Self::is_flexible_version(version)).into(),
            9 => FetchResponse9::deserialize(buf, Self::is_flexible_version(version)).into(),
            10 => FetchResponse10::deserialize(buf, Self::is_flexible_version(version)).into(),
            11 => FetchResponse11::deserialize(buf, Self::is_flexible_version(version)).into(),
            12 => FetchResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => FetchResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest0 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: Vec<FetchRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics0 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions0 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest1 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: Vec<FetchRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics1 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions1 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest2 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub topics: Vec<FetchRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics2 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions2 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest3 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub topics: Vec<FetchRequestTopics3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics3 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions3 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest4 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub topics: Vec<FetchRequestTopics4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics4 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions4 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest5 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub topics: Vec<FetchRequestTopics5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics5 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions5 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest6 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub topics: Vec<FetchRequestTopics6>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics6 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions6>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions6 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest7 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub session_id: Int32,
    pub session_epoch: Int32,
    pub topics: Vec<FetchRequestTopics7>,
    pub forgotten_topics_data: Vec<FetchRequestForgottenTopicsData7>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics7 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions7>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions7 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestForgottenTopicsData7 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest8 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub session_id: Int32,
    pub session_epoch: Int32,
    pub topics: Vec<FetchRequestTopics8>,
    pub forgotten_topics_data: Vec<FetchRequestForgottenTopicsData8>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics8 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions8>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions8 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestForgottenTopicsData8 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest9 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub session_id: Int32,
    pub session_epoch: Int32,
    pub topics: Vec<FetchRequestTopics9>,
    pub forgotten_topics_data: Vec<FetchRequestForgottenTopicsData9>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics9 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions9>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions9 {
    pub partition: Int32,
    pub current_leader_epoch: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestForgottenTopicsData9 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest10 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub session_id: Int32,
    pub session_epoch: Int32,
    pub topics: Vec<FetchRequestTopics10>,
    pub forgotten_topics_data: Vec<FetchRequestForgottenTopicsData10>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics10 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions10>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions10 {
    pub partition: Int32,
    pub current_leader_epoch: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestForgottenTopicsData10 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest11 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub session_id: Int32,
    pub session_epoch: Int32,
    pub topics: Vec<FetchRequestTopics11>,
    pub forgotten_topics_data: Vec<FetchRequestForgottenTopicsData11>,
    pub rack_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics11 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions11>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions11 {
    pub partition: Int32,
    pub current_leader_epoch: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Int64,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestForgottenTopicsData11 {
    pub topic: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest12 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Int32,
    pub isolation_level: Int8,
    pub session_id: Int32,
    pub session_epoch: Int32,
    pub topics: Vec<FetchRequestTopics12>,
    pub forgotten_topics_data: Vec<FetchRequestForgottenTopicsData12>,
    pub rack_id: String,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics12 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions12>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions12 {
    pub partition: Int32,
    pub current_leader_epoch: Int32,
    pub fetch_offset: Int64,
    pub last_fetched_epoch: Int32,
    pub log_start_offset: Int64,
    pub partition_max_bytes: Int32,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestForgottenTopicsData12 {
    pub topic: String,
    pub partitions: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse0 {
    pub responses: Vec<FetchResponseResponses0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses0 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses0 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse1 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<FetchResponseResponses1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses1 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses1 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse2 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<FetchResponseResponses2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses2 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses2 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<FetchResponseResponses3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses3 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses3 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<FetchResponseResponses4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses4 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses4 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions4>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions4 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse5 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<FetchResponseResponses5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses5 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses5 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions5>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions5 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse6 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<FetchResponseResponses6>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses6 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses6>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses6 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions6>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions6 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse7 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Option<Int16>,
    pub session_id: Option<Int32>,
    pub responses: Vec<FetchResponseResponses7>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses7 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses7>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses7 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions7>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions7 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse8 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Option<Int16>,
    pub session_id: Option<Int32>,
    pub responses: Vec<FetchResponseResponses8>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses8 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses8>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses8 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions8>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions8 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse9 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Option<Int16>,
    pub session_id: Option<Int32>,
    pub responses: Vec<FetchResponseResponses9>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses9 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses9>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses9 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions9>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions9 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse10 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Option<Int16>,
    pub session_id: Option<Int32>,
    pub responses: Vec<FetchResponseResponses10>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses10 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses10>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses10 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions10>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions10 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse11 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Option<Int16>,
    pub session_id: Option<Int32>,
    pub responses: Vec<FetchResponseResponses11>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses11 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses11>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses11 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions11>>,
    pub preferred_read_replica: Option<Int32>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions11 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse12 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Option<Int16>,
    pub session_id: Option<Int32>,
    pub responses: Vec<FetchResponseResponses12>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses12 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses12>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses12 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions12>>,
    pub preferred_read_replica: Option<Int32>,
    pub record_set: Records,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
    pub producer_id: Int64,
    pub first_offset: Int64,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<FetchRequest12> for FetchRequest0 {
    fn from(latest: FetchRequest12) -> FetchRequest0 {
        log::debug!("Using old api format - FetchRequest0, ignoring field max_bytes");
        log::debug!("Using old api format - FetchRequest0, ignoring field isolation_level");
        log::debug!("Using old api format - FetchRequest0, ignoring field session_id");
        log::debug!("Using old api format - FetchRequest0, ignoring field session_epoch");
        log::debug!("Using old api format - FetchRequest0, ignoring field forgotten_topics_data");
        FetchRequest0 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics0 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics0 {
        FetchRequestTopics0 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions0 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions0 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions0, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions0, ignoring field last_fetched_epoch");
        log::debug!(
            "Using old api format - FetchRequestTopicsPartitions0, ignoring field log_start_offset"
        );
        FetchRequestTopicsPartitions0 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequest12> for FetchRequest1 {
    fn from(latest: FetchRequest12) -> FetchRequest1 {
        log::debug!("Using old api format - FetchRequest1, ignoring field max_bytes");
        log::debug!("Using old api format - FetchRequest1, ignoring field isolation_level");
        log::debug!("Using old api format - FetchRequest1, ignoring field session_id");
        log::debug!("Using old api format - FetchRequest1, ignoring field session_epoch");
        log::debug!("Using old api format - FetchRequest1, ignoring field forgotten_topics_data");
        FetchRequest1 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics1 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics1 {
        FetchRequestTopics1 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions1 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions1 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions1, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions1, ignoring field last_fetched_epoch");
        log::debug!(
            "Using old api format - FetchRequestTopicsPartitions1, ignoring field log_start_offset"
        );
        FetchRequestTopicsPartitions1 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequest12> for FetchRequest2 {
    fn from(latest: FetchRequest12) -> FetchRequest2 {
        log::debug!("Using old api format - FetchRequest2, ignoring field max_bytes");
        log::debug!("Using old api format - FetchRequest2, ignoring field isolation_level");
        log::debug!("Using old api format - FetchRequest2, ignoring field session_id");
        log::debug!("Using old api format - FetchRequest2, ignoring field session_epoch");
        log::debug!("Using old api format - FetchRequest2, ignoring field forgotten_topics_data");
        FetchRequest2 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics2 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics2 {
        FetchRequestTopics2 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions2 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions2 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions2, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions2, ignoring field last_fetched_epoch");
        log::debug!(
            "Using old api format - FetchRequestTopicsPartitions2, ignoring field log_start_offset"
        );
        FetchRequestTopicsPartitions2 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequest12> for FetchRequest3 {
    fn from(latest: FetchRequest12) -> FetchRequest3 {
        log::debug!("Using old api format - FetchRequest3, ignoring field isolation_level");
        log::debug!("Using old api format - FetchRequest3, ignoring field session_id");
        log::debug!("Using old api format - FetchRequest3, ignoring field session_epoch");
        log::debug!("Using old api format - FetchRequest3, ignoring field forgotten_topics_data");
        FetchRequest3 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics3 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics3 {
        FetchRequestTopics3 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions3 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions3 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions3, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions3, ignoring field last_fetched_epoch");
        log::debug!(
            "Using old api format - FetchRequestTopicsPartitions3, ignoring field log_start_offset"
        );
        FetchRequestTopicsPartitions3 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequest12> for FetchRequest4 {
    fn from(latest: FetchRequest12) -> FetchRequest4 {
        log::debug!("Using old api format - FetchRequest4, ignoring field session_id");
        log::debug!("Using old api format - FetchRequest4, ignoring field session_epoch");
        log::debug!("Using old api format - FetchRequest4, ignoring field forgotten_topics_data");
        FetchRequest4 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics4 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics4 {
        FetchRequestTopics4 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions4 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions4 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions4, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions4, ignoring field last_fetched_epoch");
        log::debug!(
            "Using old api format - FetchRequestTopicsPartitions4, ignoring field log_start_offset"
        );
        FetchRequestTopicsPartitions4 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequest12> for FetchRequest5 {
    fn from(latest: FetchRequest12) -> FetchRequest5 {
        log::debug!("Using old api format - FetchRequest5, ignoring field session_id");
        log::debug!("Using old api format - FetchRequest5, ignoring field session_epoch");
        log::debug!("Using old api format - FetchRequest5, ignoring field forgotten_topics_data");
        FetchRequest5 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics5 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics5 {
        FetchRequestTopics5 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions5 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions5 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions5, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions5, ignoring field last_fetched_epoch");
        FetchRequestTopicsPartitions5 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequest12> for FetchRequest6 {
    fn from(latest: FetchRequest12) -> FetchRequest6 {
        log::debug!("Using old api format - FetchRequest6, ignoring field session_id");
        log::debug!("Using old api format - FetchRequest6, ignoring field session_epoch");
        log::debug!("Using old api format - FetchRequest6, ignoring field forgotten_topics_data");
        FetchRequest6 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics6 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics6 {
        FetchRequestTopics6 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions6 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions6 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions6, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions6, ignoring field last_fetched_epoch");
        FetchRequestTopicsPartitions6 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequest12> for FetchRequest7 {
    fn from(latest: FetchRequest12) -> FetchRequest7 {
        FetchRequest7 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            forgotten_topics_data: latest
                .forgotten_topics_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics7 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics7 {
        FetchRequestTopics7 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions7 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions7 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions7, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions7, ignoring field last_fetched_epoch");
        FetchRequestTopicsPartitions7 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData7 {
    fn from(latest: FetchRequestForgottenTopicsData12) -> FetchRequestForgottenTopicsData7 {
        FetchRequestForgottenTopicsData7 {
            topic: latest.topic,
            partitions: latest.partitions,
        }
    }
}

impl From<FetchRequest12> for FetchRequest8 {
    fn from(latest: FetchRequest12) -> FetchRequest8 {
        FetchRequest8 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            forgotten_topics_data: latest
                .forgotten_topics_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics8 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics8 {
        FetchRequestTopics8 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions8 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions8 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions8, ignoring field current_leader_epoch");
        log::debug!("Using old api format - FetchRequestTopicsPartitions8, ignoring field last_fetched_epoch");
        FetchRequestTopicsPartitions8 {
            partition: latest.partition,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData8 {
    fn from(latest: FetchRequestForgottenTopicsData12) -> FetchRequestForgottenTopicsData8 {
        FetchRequestForgottenTopicsData8 {
            topic: latest.topic,
            partitions: latest.partitions,
        }
    }
}

impl From<FetchRequest12> for FetchRequest9 {
    fn from(latest: FetchRequest12) -> FetchRequest9 {
        FetchRequest9 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            forgotten_topics_data: latest
                .forgotten_topics_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics9 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics9 {
        FetchRequestTopics9 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions9 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions9 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions9, ignoring field last_fetched_epoch");
        FetchRequestTopicsPartitions9 {
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData9 {
    fn from(latest: FetchRequestForgottenTopicsData12) -> FetchRequestForgottenTopicsData9 {
        FetchRequestForgottenTopicsData9 {
            topic: latest.topic,
            partitions: latest.partitions,
        }
    }
}

impl From<FetchRequest12> for FetchRequest10 {
    fn from(latest: FetchRequest12) -> FetchRequest10 {
        FetchRequest10 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            forgotten_topics_data: latest
                .forgotten_topics_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics10 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics10 {
        FetchRequestTopics10 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions10 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions10 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions10, ignoring field last_fetched_epoch");
        FetchRequestTopicsPartitions10 {
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData10 {
    fn from(latest: FetchRequestForgottenTopicsData12) -> FetchRequestForgottenTopicsData10 {
        FetchRequestForgottenTopicsData10 {
            topic: latest.topic,
            partitions: latest.partitions,
        }
    }
}

impl From<FetchRequest12> for FetchRequest11 {
    fn from(latest: FetchRequest12) -> FetchRequest11 {
        FetchRequest11 {
            replica_id: latest.replica_id,
            max_wait_ms: latest.max_wait_ms,
            min_bytes: latest.min_bytes,
            max_bytes: latest.max_bytes,
            isolation_level: latest.isolation_level,
            session_id: latest.session_id,
            session_epoch: latest.session_epoch,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            forgotten_topics_data: latest
                .forgotten_topics_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            rack_id: latest.rack_id,
        }
    }
}

impl From<FetchRequestTopics12> for FetchRequestTopics11 {
    fn from(latest: FetchRequestTopics12) -> FetchRequestTopics11 {
        FetchRequestTopics11 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<FetchRequestTopicsPartitions12> for FetchRequestTopicsPartitions11 {
    fn from(latest: FetchRequestTopicsPartitions12) -> FetchRequestTopicsPartitions11 {
        log::debug!("Using old api format - FetchRequestTopicsPartitions11, ignoring field last_fetched_epoch");
        FetchRequestTopicsPartitions11 {
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            fetch_offset: latest.fetch_offset,
            log_start_offset: latest.log_start_offset,
            partition_max_bytes: latest.partition_max_bytes,
        }
    }
}

impl From<FetchRequestForgottenTopicsData12> for FetchRequestForgottenTopicsData11 {
    fn from(latest: FetchRequestForgottenTopicsData12) -> FetchRequestForgottenTopicsData11 {
        FetchRequestForgottenTopicsData11 {
            topic: latest.topic,
            partitions: latest.partitions,
        }
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
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
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
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
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
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
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
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses7> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses7) -> Self {
        FetchResponseResponses12 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
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
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses8> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses8) -> Self {
        FetchResponseResponses12 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
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
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses9> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses9) -> Self {
        FetchResponseResponses12 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
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
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses10> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses10) -> Self {
        FetchResponseResponses12 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
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
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
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
            ..FetchResponse12::default()
        }
    }
}

impl From<FetchResponseResponses11> for FetchResponseResponses12 {
    fn from(older: FetchResponseResponses11) -> Self {
        FetchResponseResponses12 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..FetchResponseResponses12::default()
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
            record_set: older.record_set,
            ..FetchResponseResponsesPartitionResponses12::default()
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
            ..FetchResponseResponsesPartitionResponsesAbortedTransactions12::default()
        }
    }
}
