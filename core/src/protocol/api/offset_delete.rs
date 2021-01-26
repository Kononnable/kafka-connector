use super::prelude::*;

pub type OffsetDeleteRequest = OffsetDeleteRequest0;
pub type OffsetDeleteResponse = OffsetDeleteResponse0;
pub fn serialize_offset_delete_request(
    data: OffsetDeleteRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_offset_delete_response(version: i32, buf: &mut Bytes) -> OffsetDeleteResponse {
    match version {
        0 => OffsetDeleteResponse::deserialize(buf),
        _ => OffsetDeleteResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct OffsetDeleteRequest0 {
    pub group_id: String,
    pub topics: Vec<OffsetDeleteRequestTopics0>,
}

#[derive(Default, ToBytes)]
pub struct OffsetDeleteRequestTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetDeleteRequestTopicsPartitions0>,
}

#[derive(Default, ToBytes)]
pub struct OffsetDeleteRequestTopicsPartitions0 {
    pub partition_index: Int32,
}

#[derive(Default, FromBytes)]
pub struct OffsetDeleteResponse0 {
    pub error_code: Int16,
    pub throttle_time_ms: Int32,
    pub topics: Vec<OffsetDeleteResponseTopics0>,
}

#[derive(Default, FromBytes)]
pub struct OffsetDeleteResponseTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetDeleteResponseTopicsPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct OffsetDeleteResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}
