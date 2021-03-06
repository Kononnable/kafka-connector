use super::prelude::*;
pub type ListOffsetsRequest = ListOffsetsRequest0;
impl ApiCall for ListOffsetsRequest0 {
    type Response = ListOffsetsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        5
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ListOffsets
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
pub struct ListOffsetsRequest0 {
    #[min_version = 0]
    pub replica_id: Int32,
    #[min_version = 2]
    pub isolation_level: Int8,
    #[min_version = 0]
    pub topics: Vec<ListOffsetsRequestTopics0>,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct ListOffsetsRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<ListOffsetsRequestTopicsPartitions0>,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct ListOffsetsRequestTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 4]
    pub current_leader_epoch: Int32,
    #[min_version = 0]
    pub timestamp: Int64,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse0 {
    #[min_version = 2]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub topics: Vec<ListOffsetsResponseTopics0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<ListOffsetsResponseTopicsPartitions0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 1]
    pub timestamp: Option<Int64>,
    #[min_version = 1]
    pub offset: Option<Int64>,
    #[min_version = 4]
    pub leader_epoch: Option<Int32>,
}

impl ListOffsetsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
