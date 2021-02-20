use super::prelude::*;
pub type OffsetForLeaderEpochRequest = OffsetForLeaderEpochRequest0;
impl ApiCall for OffsetForLeaderEpochRequest0 {
    type Response = OffsetForLeaderEpochResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::OffsetForLeaderEpoch
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        false
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
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequest0 {
    #[min_version = 3]
    pub replica_id: Int32,
    #[min_version = 0]
    pub topics: Vec<OffsetForLeaderEpochRequestTopics0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions0 {
    #[min_version = 0]
    pub partition: Int32,
    #[min_version = 2]
    pub current_leader_epoch: Int32,
    #[min_version = 0]
    pub leader_epoch: Int32,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponse0 {
    #[min_version = 2]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub topics: Vec<OffsetForLeaderEpochResponseTopics0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub partition: Int32,
    #[min_version = 1]
    pub leader_epoch: Option<Int32>,
    #[min_version = 0]
    pub end_offset: Int64,
}

impl OffsetForLeaderEpochResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.topics.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl OffsetForLeaderEpochResponseTopics0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl OffsetForLeaderEpochResponseTopicsPartitions0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
