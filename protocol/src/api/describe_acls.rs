use super::prelude::*;

pub type DescribeAclsRequest = DescribeAclsRequest2;
pub type DescribeAclsResponse = DescribeAclsResponse2;
impl ApiCall for DescribeAclsRequest {
    type Response = DescribeAclsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeAcls
    }
    fn get_first_error(response: &DescribeAclsResponse) -> Option<ApiError> {
        DescribeAclsResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                DescribeAclsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DescribeAclsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &DescribeAclsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &DescribeAclsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DescribeAclsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => DescribeAclsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => DescribeAclsResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => DescribeAclsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => DescribeAclsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeAclsRequest0 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeAclsRequest1 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub pattern_type_filter: Int8,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeAclsRequest2 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub pattern_type_filter: Int8,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resources: Vec<DescribeAclsResponseResources0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponseResources0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub acls: Vec<DescribeAclsResponseResourcesAcls0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponseResourcesAcls0 {
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resources: Vec<DescribeAclsResponseResources1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponseResources1 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub pattern_type: Option<Int8>,
    pub acls: Vec<DescribeAclsResponseResourcesAcls1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponseResourcesAcls1 {
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponse2 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resources: Vec<DescribeAclsResponseResources2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponseResources2 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub pattern_type: Option<Int8>,
    pub acls: Vec<DescribeAclsResponseResourcesAcls2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeAclsResponseResourcesAcls2 {
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<DescribeAclsRequest2> for DescribeAclsRequest0 {
    fn from(latest: DescribeAclsRequest2) -> DescribeAclsRequest0 {
        log::debug!(
            "Using old api format - DescribeAclsRequest0, ignoring field pattern_type_filter"
        );
        DescribeAclsRequest0 {
            resource_type_filter: latest.resource_type_filter,
            resource_name_filter: latest.resource_name_filter,
            principal_filter: latest.principal_filter,
            host_filter: latest.host_filter,
            operation: latest.operation,
            permission_type: latest.permission_type,
        }
    }
}

impl From<DescribeAclsRequest2> for DescribeAclsRequest1 {
    fn from(latest: DescribeAclsRequest2) -> DescribeAclsRequest1 {
        DescribeAclsRequest1 {
            resource_type_filter: latest.resource_type_filter,
            resource_name_filter: latest.resource_name_filter,
            pattern_type_filter: latest.pattern_type_filter,
            principal_filter: latest.principal_filter,
            host_filter: latest.host_filter,
            operation: latest.operation,
            permission_type: latest.permission_type,
        }
    }
}

impl From<DescribeAclsResponse0> for DescribeAclsResponse2 {
    fn from(older: DescribeAclsResponse0) -> Self {
        DescribeAclsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            error_message: older.error_message,
            resources: older.resources.into_iter().map(|el| el.into()).collect(),
            ..DescribeAclsResponse2::default()
        }
    }
}

impl From<DescribeAclsResponseResources0> for DescribeAclsResponseResources2 {
    fn from(older: DescribeAclsResponseResources0) -> Self {
        DescribeAclsResponseResources2 {
            resource_type: older.resource_type,
            resource_name: older.resource_name,
            acls: older.acls.into_iter().map(|el| el.into()).collect(),
            ..DescribeAclsResponseResources2::default()
        }
    }
}

impl From<DescribeAclsResponseResourcesAcls0> for DescribeAclsResponseResourcesAcls2 {
    fn from(older: DescribeAclsResponseResourcesAcls0) -> Self {
        DescribeAclsResponseResourcesAcls2 {
            principal: older.principal,
            host: older.host,
            operation: older.operation,
            permission_type: older.permission_type,
            ..DescribeAclsResponseResourcesAcls2::default()
        }
    }
}

impl From<DescribeAclsResponse1> for DescribeAclsResponse2 {
    fn from(older: DescribeAclsResponse1) -> Self {
        DescribeAclsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            error_message: older.error_message,
            resources: older.resources.into_iter().map(|el| el.into()).collect(),
            ..DescribeAclsResponse2::default()
        }
    }
}

impl From<DescribeAclsResponseResources1> for DescribeAclsResponseResources2 {
    fn from(older: DescribeAclsResponseResources1) -> Self {
        DescribeAclsResponseResources2 {
            resource_type: older.resource_type,
            resource_name: older.resource_name,
            pattern_type: older.pattern_type,
            acls: older.acls.into_iter().map(|el| el.into()).collect(),
            ..DescribeAclsResponseResources2::default()
        }
    }
}

impl From<DescribeAclsResponseResourcesAcls1> for DescribeAclsResponseResourcesAcls2 {
    fn from(older: DescribeAclsResponseResourcesAcls1) -> Self {
        DescribeAclsResponseResourcesAcls2 {
            principal: older.principal,
            host: older.host,
            operation: older.operation,
            permission_type: older.permission_type,
            ..DescribeAclsResponseResourcesAcls2::default()
        }
    }
}

impl DescribeAclsResponse2 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.resources.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DescribeAclsResponseResources2 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.acls.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DescribeAclsResponseResourcesAcls2 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
