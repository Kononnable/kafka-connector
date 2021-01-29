use super::prelude::*;

pub type OffsetDeleteRequest = OffsetDeleteRequest0;
pub type OffsetDeleteResponse = OffsetDeleteResponse0;
impl ApiCall for OffsetDeleteRequest {
    type Response = OffsetDeleteResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::OffsetDelete
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> OffsetDeleteResponse {
        match version {
            0 => OffsetDeleteResponse::deserialize(buf),
            _ => OffsetDeleteResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, ToBytes)]
pub struct OffsetDeleteRequest0 {
    pub group_id: String,
    pub topics: Vec<OffsetDeleteRequestTopics0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct OffsetDeleteRequestTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetDeleteRequestTopicsPartitions0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct OffsetDeleteRequestTopicsPartitions0 {
    pub partition_index: Int32,
}

#[derive(Default, Debug, FromBytes)]
pub struct OffsetDeleteResponse0 {
    pub error_code: Int16,
    pub throttle_time_ms: Int32,
    pub topics: Vec<OffsetDeleteResponseTopics0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct OffsetDeleteResponseTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetDeleteResponseTopicsPartitions0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct OffsetDeleteResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}
