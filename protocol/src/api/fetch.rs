use super::prelude::*;
pub type FetchRequest = FetchRequest0;
impl ApiCall for FetchRequest0 {
    type Response = FetchResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        12
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::Fetch
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 12
    }
    fn serialize(&self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(self, buf, Self::is_flexible_version(version), version);
    }
    fn deserialize_response(version: u16, buf: &mut Bytes) -> (i32, Self::Response) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse::deserialize(buf, false, 2).correlation,
            false => HeaderResponse::deserialize(buf, false, 1).correlation,
        };
        let response =
            Self::Response::deserialize(buf, Self::is_flexible_version(version), version);
        (correlation, response)
    }
    fn deserialize_request(version: u16, buf: &mut Bytes) -> (OwnedHeaderRequest, Self) {
        let header = match Self::is_flexible_version(version) {
            true => OwnedHeaderRequest::deserialize(buf, false, 2),
            false => OwnedHeaderRequest::deserialize(buf, false, 1),
        };
        let request = Self::deserialize(buf, Self::is_flexible_version(version), version);
        (header, request)
    }
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct FetchRequest0 {
    #[min_version = 0]
    pub replica_id: Int32,
    #[min_version = 0]
    pub max_wait_ms: Int32,
    #[min_version = 0]
    pub min_bytes: Int32,
    #[min_version = 3]
    pub max_bytes: Int32,
    #[min_version = 4]
    pub isolation_level: Int8,
    #[min_version = 7]
    pub session_id: Int32,
    #[min_version = 7]
    pub session_epoch: Int32,
    #[min_version = 0]
    pub topics: Vec<FetchRequestTopics0>,
    #[min_version = 7]
    pub forgotten_topics_data: Vec<FetchRequestForgottenTopicsData0>,
    #[min_version = 11]
    pub rack_id: String,
    #[min_version = 12]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct FetchRequestTopics0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub partitions: Vec<FetchRequestTopicsPartitions0>,
    #[min_version = 12]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct FetchRequestTopicsPartitions0 {
    #[min_version = 0]
    pub partition: Int32,
    #[min_version = 9]
    pub current_leader_epoch: Int32,
    #[min_version = 0]
    pub fetch_offset: Int64,
    #[min_version = 12]
    pub last_fetched_epoch: Int32,
    #[min_version = 5]
    pub log_start_offset: Int64,
    #[min_version = 0]
    pub partition_max_bytes: Int32,
    #[min_version = 12]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct FetchRequestForgottenTopicsData0 {
    #[min_version = 7]
    pub topic: String,
    #[min_version = 7]
    pub partitions: Vec<Int32>,
    #[min_version = 12]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponse0 {
    #[min_version = 1]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 7]
    pub error_code: Option<Int16>,
    #[min_version = 7]
    pub session_id: Option<Int32>,
    #[min_version = 0]
    pub responses: Vec<FetchResponseResponses0>,
    #[min_version = 12]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponses0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub partition_responses: Vec<FetchResponseResponsesPartitionResponses0>,
    #[min_version = 12]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponses0 {
    #[min_version = 0]
    pub partition: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub high_watermark: Int64,
    #[min_version = 4]
    pub last_stable_offset: Option<Int64>,
    #[min_version = 5]
    pub log_start_offset: Option<Int64>,
    #[min_version = 4]
    pub aborted_transactions:
        Option<Vec<FetchResponseResponsesPartitionResponsesAbortedTransactions0>>,
    #[min_version = 11]
    pub preferred_read_replica: Option<Int32>,
    #[min_version = 0]
    pub record_set: Records,
    #[min_version = 12]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct FetchResponseResponsesPartitionResponsesAbortedTransactions0 {
    #[min_version = 4]
    pub producer_id: Option<Int64>,
    #[min_version = 4]
    pub first_offset: Option<Int64>,
    #[min_version = 12]
    pub tag_buffer: Option<TagBuffer>,
}

impl FetchResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.responses.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl FetchResponseResponses0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partition_responses.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl FetchResponseResponsesPartitionResponses0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if let Some(vec) = self.aborted_transactions.as_ref() {
            for item in vec {
                if let Some(x) = item.get_first_error() {
                    return Some(x);
                };
            }
        }
        None
    }
}
impl FetchResponseResponsesPartitionResponsesAbortedTransactions0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
