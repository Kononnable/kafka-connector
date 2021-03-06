use super::prelude::*;
pub type OffsetFetchRequest = OffsetFetchRequest0;
impl ApiCall for OffsetFetchRequest0 {
    type Response = OffsetFetchResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        7
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::OffsetFetch
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 6
    }
    fn serialize(&self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(self, buf, Self::is_flexible_version(version), version);
    }
    fn deserialize_response(version: u16, buf: &mut Bytes) -> (i32, Self::Response) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse::deserialize(buf, false, 2).correlation,
            false => HeaderResponse::deserialize(buf, false, 1).correlation,
        };
        let response =
            Self::Response::deserialize(buf, Self::is_flexible_version(version), version);
        (correlation, response)
    }
    fn deserialize_request(version: u16, buf: &mut Bytes) -> (OwnedHeaderRequest, Self) {
        let header = match Self::is_flexible_version(version) {
            true => OwnedHeaderRequest::deserialize(buf, false, 2),
            false => OwnedHeaderRequest::deserialize(buf, false, 1),
        };
        let request = Self::deserialize(buf, Self::is_flexible_version(version), version);
        (header, request)
    }
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct OffsetFetchRequest0 {
    #[min_version = 0]
    pub group_id: String,
    #[min_version = 0]
    pub topics: Vec<OffsetFetchRequestTopics0>,
    #[min_version = 7]
    pub require_stable: Boolean,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct OffsetFetchRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partition_indexes: Vec<Int32>,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse0 {
    #[min_version = 3]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub topics: Vec<OffsetFetchResponseTopics0>,
    #[min_version = 2]
    pub error_code: Option<Int16>,
    #[min_version = 6]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions0>,
    #[min_version = 6]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub committed_offset: Int64,
    #[min_version = 5]
    pub committed_leader_epoch: Option<Int32>,
    #[min_version = 0]
    pub metadata: NullableString,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 6]
    pub tag_buffer: Option<TagBuffer>,
}

impl OffsetFetchResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if self.error_code.is_some() && self.error_code.unwrap() != 0 {
            return self.error_code.map(ApiError::from);
        }
        None
    }
}
