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
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            _ => false,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                OffsetDeleteRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                OffsetDeleteRequest::get_api_key(),
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
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, OffsetDeleteResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => OffsetDeleteResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => OffsetDeleteResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetDeleteRequest0 {
    pub group_id: String,
    pub topics: Vec<OffsetDeleteRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetDeleteRequestTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetDeleteRequestTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetDeleteRequestTopicsPartitions0 {
    pub partition_index: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetDeleteResponse0 {
    pub error_code: Int16,
    pub throttle_time_ms: Int32,
    pub topics: Vec<OffsetDeleteResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetDeleteResponseTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetDeleteResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetDeleteResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}
