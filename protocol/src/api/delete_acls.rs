use super::prelude::*;
pub type DeleteAclsRequest = DeleteAclsRequest0;
impl ApiCall for DeleteAclsRequest0 {
    type Response = DeleteAclsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DeleteAcls
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
pub struct DeleteAclsRequest0 {
    #[min_version = 0]
    pub filters: Vec<DeleteAclsRequestFilters0>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct DeleteAclsRequestFilters0 {
    #[min_version = 0]
    pub resource_type_filter: Int8,
    #[min_version = 0]
    pub resource_name_filter: NullableString,
    #[min_version = 1]
    pub pattern_type_filter: Int8,
    #[min_version = 0]
    pub principal_filter: NullableString,
    #[min_version = 0]
    pub host_filter: NullableString,
    #[min_version = 0]
    pub operation: Int8,
    #[min_version = 0]
    pub permission_type: Int8,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub filter_results: Vec<DeleteAclsResponseFilterResults0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponseFilterResults0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub matching_acls: Vec<DeleteAclsResponseFilterResultsMatchingAcls0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponseFilterResultsMatchingAcls0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub resource_type: Int8,
    #[min_version = 0]
    pub resource_name: String,
    #[min_version = 1]
    pub pattern_type: Option<Int8>,
    #[min_version = 0]
    pub principal: String,
    #[min_version = 0]
    pub host: String,
    #[min_version = 0]
    pub operation: Int8,
    #[min_version = 0]
    pub permission_type: Int8,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}

impl DeleteAclsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.filter_results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DeleteAclsResponseFilterResults0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.matching_acls.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DeleteAclsResponseFilterResultsMatchingAcls0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
