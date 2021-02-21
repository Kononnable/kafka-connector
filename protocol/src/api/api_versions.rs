use super::prelude::*;
pub type ApiVersionsRequest = ApiVersionsRequest0;
impl ApiCall for ApiVersionsRequest0 {
    type Response = ApiVersionsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ApiVersions
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 3
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
pub struct ApiVersionsRequest0 {
    #[min_version = 3]
    pub client_software_name: String,
    #[min_version = 3]
    pub client_software_version: String,
    #[min_version = 3]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponse0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub api_keys: Vec<ApiVersionsResponseApiKeys0>,
    #[min_version = 1]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 3]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponseApiKeys0 {
    #[min_version = 0]
    pub api_key: Int16,
    #[min_version = 0]
    pub min_version: Int16,
    #[min_version = 0]
    pub max_version: Int16,
    #[min_version = 3]
    pub tag_buffer: Option<TagBuffer>,
}

impl ApiVersionsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.api_keys.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl ApiVersionsResponseApiKeys0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
