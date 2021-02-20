use super::prelude::*;
pub type OffsetCommitRequest = OffsetCommitRequest0;
impl ApiCall for OffsetCommitRequest0 {
    type Response = OffsetCommitResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        8
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::OffsetCommit
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 8
    }
    fn serialize(self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(&self, buf, Self::is_flexible_version(version), version);
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
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest0 {
    #[min_version = 0]
    pub group_id: String,
    #[min_version = 1]
    pub generation_id: Int32,
    #[min_version = 1]
    pub member_id: String,
    #[min_version = 7]
    pub group_instance_id: NullableString,
    #[min_version = 0]
    pub topics: Vec<OffsetCommitRequestTopics0>,
    #[min_version = 8]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions0>,
    #[min_version = 8]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub committed_offset: Int64,
    #[min_version = 6]
    pub committed_leader_epoch: Int32,
    #[min_version = 0]
    pub committed_metadata: NullableString,
    #[min_version = 8]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse0 {
    #[min_version = 3]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub topics: Vec<OffsetCommitResponseTopics0>,
    #[min_version = 8]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions0>,
    #[min_version = 8]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 8]
    pub tag_buffer: Option<TagBuffer>,
}

impl OffsetCommitResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.topics.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl OffsetCommitResponseTopics0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl OffsetCommitResponseTopicsPartitions0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
