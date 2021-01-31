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
                &FetchRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &FetchRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &FetchRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &FetchRequest3::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &FetchRequest4::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &FetchRequest5::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(
                &FetchRequest6::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            7 => ToBytes::serialize(
                &FetchRequest7::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            8 => ToBytes::serialize(
                &FetchRequest8::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            9 => ToBytes::serialize(
                &FetchRequest9::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            10 => ToBytes::serialize(
                &FetchRequest10::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            11 => ToBytes::serialize(
                &FetchRequest11::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            12 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, FetchResponse) {
        let header = HeaderResponse::deserialize(buf, false);
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
        (header.correlation, response)
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
    pub max_bytes: Optional<Int32>,
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
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
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
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
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
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequest6 {
    pub replica_id: Int32,
    pub max_wait_ms: Int32,
    pub min_bytes: Int32,
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
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
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
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

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics7 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions7>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions7 {
    pub partition: Int32,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
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
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics8>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData8>>,
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
    pub log_start_offset: Optional<Int64>,
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
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics9>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData9>>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics9 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions9>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions9 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
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
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics10>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData10>>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics10 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions10>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions10 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
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
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics11>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData11>>,
    pub rack_id: Optional<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics11 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions11>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions11 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub log_start_offset: Optional<Int64>,
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
    pub max_bytes: Optional<Int32>,
    pub isolation_level: Optional<Int8>,
    pub session_id: Optional<Int32>,
    pub session_epoch: Optional<Int32>,
    pub topics: Vec<FetchRequestTopics12>,
    pub forgotten_topics_data: Optional<Vec<FetchRequestForgottenTopicsData12>>,
    pub rack_id: Optional<String>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopics12 {
    pub topic: String,
    pub partitions: Vec<FetchRequestTopicsPartitions12>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestTopicsPartitions12 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub fetch_offset: Int64,
    pub last_fetched_epoch: Optional<Int32>,
    pub log_start_offset: Optional<Int64>,
    pub partition_max_bytes: Int32,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FetchRequestForgottenTopicsData12 {
    pub topic: String,
    pub partitions: Vec<Int32>,
    pub tag_buffer: Optional<TagBuffer>,
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
    pub throttle_time_ms: Optional<Int32>,
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
    pub throttle_time_ms: Optional<Int32>,
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
    pub throttle_time_ms: Optional<Int32>,
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
    pub throttle_time_ms: Optional<Int32>,
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
    pub last_stable_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions4>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions4 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse5 {
    pub throttle_time_ms: Optional<Int32>,
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
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions5>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions5 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse6 {
    pub throttle_time_ms: Optional<Int32>,
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
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions6>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions6 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse7 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
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
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions7>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions7 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse8 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
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
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions8>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions8 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse9 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
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
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions9>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions9 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse10 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
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
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions10>>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions10 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse11 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
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
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions11>>,
    pub preferred_read_replica: Optional<Int32>,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions11 {
    pub producer_id: Int64,
    pub first_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse12 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Optional<Int16>,
    pub session_id: Optional<Int32>,
    pub responses: Vec<FetchResponseResponses12>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses12 {
    pub topic: String,
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses12>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses12 {
    pub partition: Int32,
    pub error_code: Int16,
    pub high_watermark: Int64,
    pub last_stable_offset: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub aborted_transactions:
        Optional<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions12>>,
    pub preferred_read_replica: Optional<Int32>,
    pub record_set: Records,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions12 {
    pub producer_id: Int64,
    pub first_offset: Int64,
    pub tag_buffer: Optional<TagBuffer>,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 0, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                0,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics0 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                0,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 1, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                1,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics1 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                1,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 2, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                2,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics2 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                2,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 3, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                3,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics3 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                3,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 4, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                4,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics4 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                4,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 5, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                5,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics5 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                5,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 6, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                6,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics6 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                6,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 7, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                7,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics7 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                7,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestForgottenTopicsData",
                7,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestForgottenTopicsData7 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 8, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                8,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics8 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                8,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestForgottenTopicsData",
                8,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestForgottenTopicsData8 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 9, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                9,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics9 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                9,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestForgottenTopicsData",
                9,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestForgottenTopicsData9 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 10, "tag_buffer"));
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                10,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics10 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                10,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestForgottenTopicsData",
                10,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestForgottenTopicsData10 {
            topic: latest.topic,
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<FetchRequest12> for FetchRequest11 {
    type Error = Error;
    fn try_from(latest: FetchRequest12) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("FetchRequest", 11, "tag_buffer"));
        }
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
            rack_id: latest.rack_id,
        })
    }
}

impl TryFrom<FetchRequestTopics12> for FetchRequestTopics11 {
    type Error = Error;
    fn try_from(latest: FetchRequestTopics12) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopics",
                11,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestTopics11 {
            topic: latest.topic,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestTopicsPartitions",
                11,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "FetchRequestForgottenTopicsData",
                11,
                "tag_buffer",
            ));
        }
        Ok(FetchRequestForgottenTopicsData11 {
            topic: latest.topic,
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
