use super::prelude::*;

pub type AlterIsrRequest = AlterIsrRequest0;
pub type AlterIsrResponse = AlterIsrResponse0;
pub fn serialize_alter_isr_request(
    data: AlterIsrRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_alter_isr_response<T>(version: i32, buf: &mut T) -> AlterIsrResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        1 => AlterIsrResponse::deserialize(buf),
        _ => AlterIsrResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct AlterIsrRequest0 {
    pub broker_id: Int32,
    pub broker_epoch: Int64,
    pub topics: Vec<AlterIsrRequestTopics0>,
}

#[derive(Default, ToBytes)]
pub struct AlterIsrRequestTopics0 {
    pub name: CompactString,
    pub partitions: Vec<AlterIsrRequestTopicsPartitions0>,
}

#[derive(Default, ToBytes)]
pub struct AlterIsrRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub leader_epoch: Int32,
    pub new_isr: Vec<Int32>,
    pub current_isr_version: Int32,
}

#[derive(Default, FromBytes)]
pub struct AlterIsrResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub topics: Vec<AlterIsrResponseTopics0>,
}

#[derive(Default, FromBytes)]
pub struct AlterIsrResponseTopics0 {
    pub name: CompactString,
    pub partitions: Vec<AlterIsrResponseTopicsPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct AlterIsrResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub leader_id: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub current_isr_version: Int32,
}
