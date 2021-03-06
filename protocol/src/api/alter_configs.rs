use super::prelude::*;
pub type AlterConfigsRequest = AlterConfigsRequest0;
impl ApiCall for AlterConfigsRequest0 {
    type Response = AlterConfigsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        1
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterConfigs
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
pub struct AlterConfigsRequest0 {
    #[min_version = 0]
    pub resources: Vec<AlterConfigsRequestResources0>,
    #[min_version = 0]
    pub validate_only: Boolean,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct AlterConfigsRequestResources0 {
    #[min_version = 0]
    pub resource_type: Int8,
    #[min_version = 0]
    pub resource_name: String,
    #[min_version = 0]
    pub configs: Vec<AlterConfigsRequestResourcesConfigs0>,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct AlterConfigsRequestResourcesConfigs0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub value: NullableString,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterConfigsResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub responses: Vec<AlterConfigsResponseResponses0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterConfigsResponseResponses0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub resource_type: Int8,
    #[min_version = 0]
    pub resource_name: String,
}

impl AlterConfigsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
