use super::prelude::*;

pub type AlterPartitionReassignmentsRequest = AlterPartitionReassignmentsRequest0;
pub type AlterPartitionReassignmentsResponse = AlterPartitionReassignmentsResponse0;
pub fn serialize_alter_partition_reassignments_request(
    data: AlterPartitionReassignmentsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_alter_partition_reassignments_response(
    version: i32,
    buf: &mut Bytes,
) -> AlterPartitionReassignmentsResponse {
    match version {
        0 => AlterPartitionReassignmentsResponse::deserialize(buf),
        _ => AlterPartitionReassignmentsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct AlterPartitionReassignmentsRequest0 {
    pub timeout_ms: Int32,
    pub topics: Vec<AlterPartitionReassignmentsRequestTopics0>,
}

#[derive(Default, ToBytes)]
pub struct AlterPartitionReassignmentsRequestTopics0 {
    pub name: CompactString,
    pub partitions: Vec<AlterPartitionReassignmentsRequestTopicsPartitions0>,
}

#[derive(Default, ToBytes)]
pub struct AlterPartitionReassignmentsRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub replicas: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct AlterPartitionReassignmentsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub responses: Vec<AlterPartitionReassignmentsResponseResponses0>,
}

#[derive(Default, FromBytes)]
pub struct AlterPartitionReassignmentsResponseResponses0 {
    pub name: CompactString,
    pub partitions: Vec<AlterPartitionReassignmentsResponseResponsesPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct AlterPartitionReassignmentsResponseResponsesPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
}
