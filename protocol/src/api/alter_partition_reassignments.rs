use super::prelude::*;
pub type AlterPartitionReassignmentsRequest = AlterPartitionReassignmentsRequest0;
impl ApiCall for AlterPartitionReassignmentsRequest0 {
    type Response = AlterPartitionReassignmentsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterPartitionReassignments
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        true
    }
    fn serialize(self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(&self, buf, Self::is_flexible_version(version), version);
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
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterPartitionReassignmentsRequest0 {
    #[min_version = 0]
    pub timeout_ms: Int32,
    #[min_version = 0]
    pub topics: Vec<AlterPartitionReassignmentsRequestTopics0>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterPartitionReassignmentsRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<AlterPartitionReassignmentsRequestTopicsPartitions0>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterPartitionReassignmentsRequestTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub replicas: Vec<Int32>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterPartitionReassignmentsResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub responses: Vec<AlterPartitionReassignmentsResponseResponses0>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterPartitionReassignmentsResponseResponses0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<AlterPartitionReassignmentsResponseResponsesPartitions0>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterPartitionReassignmentsResponseResponsesPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}

impl AlterPartitionReassignmentsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.responses.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AlterPartitionReassignmentsResponseResponses0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AlterPartitionReassignmentsResponseResponsesPartitions0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
