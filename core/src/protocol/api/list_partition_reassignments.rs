use super::prelude::*;

pub type ListPartitionReassignmentsRequest = ListPartitionReassignmentsRequest0;
pub type ListPartitionReassignmentsResponse = ListPartitionReassignmentsResponse0;
pub fn serialize_list_partition_reassignments_request(
    data: ListPartitionReassignmentsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_list_partition_reassignments_response<T>(
    version: i32,
    buf: &mut T,
) -> ListPartitionReassignmentsResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        1 => ListPartitionReassignmentsResponse::deserialize(buf),
        _ => ListPartitionReassignmentsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct ListPartitionReassignmentsRequest0 {
    pub timeout_ms: Int32,
    pub topics: Vec<ListPartitionReassignmentsRequestTopics0>,
}

#[derive(Default, ToBytes)]
pub struct ListPartitionReassignmentsRequestTopics0 {
    pub name: CompactString,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ListPartitionReassignmentsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub topics: Vec<ListPartitionReassignmentsResponseTopics0>,
}

#[derive(Default, FromBytes)]
pub struct ListPartitionReassignmentsResponseTopics0 {
    pub name: CompactString,
    pub partitions: Vec<ListPartitionReassignmentsResponseTopicsPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct ListPartitionReassignmentsResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub replicas: Vec<Int32>,
    pub adding_replicas: Vec<Int32>,
    pub removing_replicas: Vec<Int32>,
}
