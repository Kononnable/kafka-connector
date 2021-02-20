use super::prelude::*;
pub type CreateTopicsRequest = CreateTopicsRequest0;
impl ApiCall for CreateTopicsRequest0 {
    type Response = CreateTopicsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        6
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::CreateTopics
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 5
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
pub struct CreateTopicsRequest0 {
    #[min_version = 0]
    pub topics: Vec<CreateTopicsRequestTopics0>,
    #[min_version = 0]
    pub timeout_ms: Int32,
    #[min_version = 1]
    pub validate_only: Boolean,
    #[min_version = 5]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub num_partitions: Int32,
    #[min_version = 0]
    pub replication_factor: Int16,
    #[min_version = 0]
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments0>,
    #[min_version = 0]
    pub configs: Vec<CreateTopicsRequestTopicsConfigs0>,
    #[min_version = 5]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub broker_ids: Vec<Int32>,
    #[min_version = 5]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub value: NullableString,
    #[min_version = 5]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponse0 {
    #[min_version = 2]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub topics: Vec<CreateTopicsResponseTopics0>,
    #[min_version = 5]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 1]
    pub error_message: Option<NullableString>,
    #[min_version = 5]
    pub num_partitions: Option<Int32>,
    #[min_version = 5]
    pub replication_factor: Option<Int16>,
    #[min_version = 5]
    pub configs: Option<Vec<CreateTopicsResponseTopicsConfigs0>>,
    #[min_version = 5]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopicsConfigs0 {
    #[min_version = 5]
    pub name: Option<String>,
    #[min_version = 5]
    pub value: Option<NullableString>,
    #[min_version = 5]
    pub read_only: Option<Boolean>,
    #[min_version = 5]
    pub config_source: Option<Int8>,
    #[min_version = 5]
    pub is_sensitive: Option<Boolean>,
    #[min_version = 5]
    pub tag_buffer: Option<TagBuffer>,
}

impl CreateTopicsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.topics.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl CreateTopicsResponseTopics0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if let Some(vec) = self.configs.as_ref() {
            for item in vec {
                if let Some(x) = item.get_first_error() {
                    return Some(x);
                };
            }
        }
        None
    }
}
impl CreateTopicsResponseTopicsConfigs0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
