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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> ListPartitionReassignmentsResponse {
        match version {
            0 => ListPartitionReassignmentsResponse::deserialize(buf),
            _ => ListPartitionReassignmentsResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, ToBytes)]
pub struct ListPartitionReassignmentsRequest0 {
    pub timeout_ms: Int32,
    pub topics: Vec<ListPartitionReassignmentsRequestTopics0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct ListPartitionReassignmentsRequestTopics0 {
    pub name: CompactString,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListPartitionReassignmentsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub topics: Vec<ListPartitionReassignmentsResponseTopics0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListPartitionReassignmentsResponseTopics0 {
    pub name: CompactString,
    pub partitions: Vec<ListPartitionReassignmentsResponseTopicsPartitions0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListPartitionReassignmentsResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub replicas: Vec<Int32>,
    pub adding_replicas: Vec<Int32>,
    pub removing_replicas: Vec<Int32>,
}
