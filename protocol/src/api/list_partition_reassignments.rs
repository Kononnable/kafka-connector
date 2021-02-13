use super::prelude::*;

pub type ListPartitionReassignmentsRequest = ListPartitionReassignmentsRequest0;
pub type ListPartitionReassignmentsResponse = ListPartitionReassignmentsResponse0;
impl ApiCall for ListPartitionReassignmentsRequest {
    type Response = ListPartitionReassignmentsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ListPartitionReassignments
    }
    fn get_first_error(response: &ListPartitionReassignmentsResponse) -> Option<ApiError> {
        ListPartitionReassignmentsResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                ListPartitionReassignmentsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                ListPartitionReassignmentsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(
        version: i16,
        buf: &mut Bytes,
    ) -> (i32, ListPartitionReassignmentsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => ListPartitionReassignmentsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
            _ => ListPartitionReassignmentsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListPartitionReassignmentsRequest0 {
    pub timeout_ms: Int32,
    pub topics: Vec<ListPartitionReassignmentsRequestTopics0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListPartitionReassignmentsRequestTopics0 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListPartitionReassignmentsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub topics: Vec<ListPartitionReassignmentsResponseTopics0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListPartitionReassignmentsResponseTopics0 {
    pub name: String,
    pub partitions: Vec<ListPartitionReassignmentsResponseTopicsPartitions0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListPartitionReassignmentsResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub replicas: Vec<Int32>,
    pub adding_replicas: Vec<Int32>,
    pub removing_replicas: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

impl ListPartitionReassignmentsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.topics.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl ListPartitionReassignmentsResponseTopics0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl ListPartitionReassignmentsResponseTopicsPartitions0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
