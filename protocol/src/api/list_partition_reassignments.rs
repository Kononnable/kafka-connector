use super::prelude::*;
pub type ListPartitionReassignmentsRequest = ListPartitionReassignmentsRequest0;
impl ApiCall for ListPartitionReassignmentsRequest0 {
    type Response = ListPartitionReassignmentsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ListPartitionReassignments
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        true
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
pub struct ListPartitionReassignmentsRequest0 {
    #[min_version = 0]
    pub timeout_ms: Int32,
    #[min_version = 0]
    pub topics: Vec<ListPartitionReassignmentsRequestTopics0>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct ListPartitionReassignmentsRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partition_indexes: Vec<Int32>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListPartitionReassignmentsResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub topics: Vec<ListPartitionReassignmentsResponseTopics0>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListPartitionReassignmentsResponseTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<ListPartitionReassignmentsResponseTopicsPartitions0>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListPartitionReassignmentsResponseTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub replicas: Vec<Int32>,
    #[min_version = 0]
    pub adding_replicas: Vec<Int32>,
    #[min_version = 0]
    pub removing_replicas: Vec<Int32>,
    #[min_version = 0]
    pub tag_buffer: TagBuffer,
}

impl ListPartitionReassignmentsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if self.error_code != 0 {
            return Some(self.error_code.into());
        }
        None
    }
}
