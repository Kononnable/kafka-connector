use super::prelude::*;
pub type DescribeAclsRequest = DescribeAclsRequest0;
impl ApiCall for DescribeAclsRequest0 {
    type Response = DescribeAclsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeAcls
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 2
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
pub struct DescribeAclsRequest0 {
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
pub struct DescribeAclsResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub resources: Vec<DescribeAclsResponseResources0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponseResources0 {
    #[min_version = 0]
    pub resource_type: Int8,
    #[min_version = 0]
    pub resource_name: String,
    #[min_version = 1]
    pub pattern_type: Option<Int8>,
    #[min_version = 0]
    pub acls: Vec<DescribeAclsResponseResourcesAcls0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponseResourcesAcls0 {
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

impl DescribeAclsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.resources.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DescribeAclsResponseResources0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.acls.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DescribeAclsResponseResourcesAcls0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
