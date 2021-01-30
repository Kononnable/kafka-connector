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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> AlterPartitionReassignmentsResponse {
        match version {
            0 => AlterPartitionReassignmentsResponse::deserialize(buf),
            _ => AlterPartitionReassignmentsResponse::deserialize(buf),
        }
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
    pub name: CompactString,
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
    pub error_message: CompactNullableString,
    pub responses: Vec<AlterPartitionReassignmentsResponseResponses0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterPartitionReassignmentsResponseResponses0 {
    pub name: CompactString,
    pub partitions: Vec<AlterPartitionReassignmentsResponseResponsesPartitions0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterPartitionReassignmentsResponseResponsesPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub tag_buffer: TagBuffer,
}
