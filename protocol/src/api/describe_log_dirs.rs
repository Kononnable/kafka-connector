use super::prelude::*;
pub type DescribeLogDirsRequest = DescribeLogDirsRequest0;
impl ApiCall for DescribeLogDirsRequest0 {
    type Response = DescribeLogDirsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeLogDirs
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 2
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
pub struct DescribeLogDirsRequest0 {
    #[min_version = 0]
    pub topics: Vec<DescribeLogDirsRequestTopics0>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct DescribeLogDirsRequestTopics0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub partition_index: Vec<Int32>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub results: Vec<DescribeLogDirsResponseResults0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResults0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub log_dir: String,
    #[min_version = 0]
    pub topics: Vec<DescribeLogDirsResponseResultsTopics0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<DescribeLogDirsResponseResultsTopicsPartitions0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub partition_size: Int64,
    #[min_version = 0]
    pub offset_lag: Int64,
    #[min_version = 0]
    pub is_future_key: Boolean,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}

impl DescribeLogDirsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
