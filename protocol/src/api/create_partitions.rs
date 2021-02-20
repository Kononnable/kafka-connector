use super::prelude::*;
pub type CreatePartitionsRequest = CreatePartitionsRequest0;
impl ApiCall for CreatePartitionsRequest0 {
    type Response = CreatePartitionsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::CreatePartitions
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
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequest0 {
    #[min_version = 0]
    pub topics: Vec<CreatePartitionsRequestTopics0>,
    #[min_version = 0]
    pub timeout_ms: Int32,
    #[min_version = 0]
    pub validate_only: Boolean,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub count: Int32,
    #[min_version = 0]
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments0>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments0 {
    #[min_version = 0]
    pub broker_ids: Vec<Int32>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub results: Vec<CreatePartitionsResponseResults0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponseResults0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}

impl CreatePartitionsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl CreatePartitionsResponseResults0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
