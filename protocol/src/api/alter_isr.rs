use super::prelude::*;

pub type AlterIsrRequest = AlterIsrRequest0;
pub type AlterIsrResponse = AlterIsrResponse0;
impl ApiCall for AlterIsrRequest {
    type Response = AlterIsrResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterIsr
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> AlterIsrResponse {
        match version {
            0 => AlterIsrResponse::deserialize(buf),
            _ => AlterIsrResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterIsrRequest0 {
    pub broker_id: Int32,
    pub broker_epoch: Int64,
    pub topics: Vec<AlterIsrRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterIsrRequestTopics0 {
    pub name: CompactString,
    pub partitions: Vec<AlterIsrRequestTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterIsrRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub leader_epoch: Int32,
    pub new_isr: Vec<Int32>,
    pub current_isr_version: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterIsrResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub topics: Vec<AlterIsrResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterIsrResponseTopics0 {
    pub name: CompactString,
    pub partitions: Vec<AlterIsrResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterIsrResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub leader_id: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub current_isr_version: Int32,
}
