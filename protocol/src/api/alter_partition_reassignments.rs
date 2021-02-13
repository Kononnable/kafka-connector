use super::prelude::*;

pub type AlterPartitionReassignmentsRequest = AlterPartitionReassignmentsRequest0;
pub type AlterPartitionReassignmentsResponse = AlterPartitionReassignmentsResponse0;
impl ApiCall for AlterPartitionReassignmentsRequest {
    type Response = AlterPartitionReassignmentsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterPartitionReassignments
    }
    fn get_first_error(response: &AlterPartitionReassignmentsResponse) -> Option<ApiError> {
        AlterPartitionReassignmentsResponse::get_first_error(response)
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
                AlterPartitionReassignmentsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                AlterPartitionReassignmentsRequest::get_api_key(),
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
    ) -> (i32, AlterPartitionReassignmentsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => AlterPartitionReassignmentsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
            _ => AlterPartitionReassignmentsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterPartitionReassignmentsRequest0 {
    pub timeout_ms: Int32,
    pub topics: Vec<AlterPartitionReassignmentsRequestTopics0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterPartitionReassignmentsRequestTopics0 {
    pub name: String,
    pub partitions: Vec<AlterPartitionReassignmentsRequestTopicsPartitions0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterPartitionReassignmentsRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub replicas: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterPartitionReassignmentsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub responses: Vec<AlterPartitionReassignmentsResponseResponses0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterPartitionReassignmentsResponseResponses0 {
    pub name: String,
    pub partitions: Vec<AlterPartitionReassignmentsResponseResponsesPartitions0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterPartitionReassignmentsResponseResponsesPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
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
