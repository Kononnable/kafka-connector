use super::prelude::*;

pub type DescribeAclsRequest = DescribeAclsRequest2;
pub type DescribeAclsResponse = DescribeAclsResponse2;
pub fn serialize_describe_acls_request(
    data: DescribeAclsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&DescribeAclsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&DescribeAclsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_describe_acls_response(version: i32, buf: &mut Bytes) -> DescribeAclsResponse {
    match version {
        0 => DescribeAclsResponse0::deserialize(buf).into(),
        1 => DescribeAclsResponse1::deserialize(buf).into(),
        2 => DescribeAclsResponse::deserialize(buf),
        _ => DescribeAclsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct DescribeAclsRequest0 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, ToBytes)]
pub struct DescribeAclsRequest1 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub pattern_type_filter: Optional<Int8>,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, ToBytes)]
pub struct DescribeAclsRequest2 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: CompactNullableString,
    pub pattern_type_filter: Optional<Int8>,
    pub principal_filter: CompactNullableString,
    pub host_filter: CompactNullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resources: Vec<DescribeAclsResponseResources0>,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponseResources0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub acls: Vec<DescribeAclsResponseResourcesAcls0>,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponseResourcesAcls0 {
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resources: Vec<DescribeAclsResponseResources1>,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponseResources1 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub pattern_type: Optional<Int8>,
    pub acls: Vec<DescribeAclsResponseResourcesAcls1>,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponseResourcesAcls1 {
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponse2 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub resources: Vec<DescribeAclsResponseResources2>,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponseResources2 {
    pub resource_type: Int8,
    pub resource_name: CompactString,
    pub pattern_type: Optional<Int8>,
    pub acls: Vec<DescribeAclsResponseResourcesAcls2>,
}

#[derive(Default, FromBytes)]
pub struct DescribeAclsResponseResourcesAcls2 {
    pub principal: CompactString,
    pub host: CompactString,
    pub operation: Int8,
    pub permission_type: Int8,
}

impl TryFrom<DescribeAclsRequest2> for DescribeAclsRequest0 {
    type Error = Error;
    fn try_from(latest: DescribeAclsRequest2) -> Result<Self, Self::Error> {
        if latest.pattern_type_filter.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeAclsRequest",
                0,
                "pattern_type_filter",
            ));
        }
        Ok(DescribeAclsRequest0 {
            resource_type_filter: latest.resource_type_filter,
            resource_name_filter: latest.resource_name_filter,
            principal_filter: latest.principal_filter,
            host_filter: latest.host_filter,
            operation: latest.operation,
            permission_type: latest.permission_type,
        })
    }
}

impl TryFrom<DescribeAclsRequest2> for DescribeAclsRequest1 {
    type Error = Error;
    fn try_from(latest: DescribeAclsRequest2) -> Result<Self, Self::Error> {
        Ok(DescribeAclsRequest1 {
            resource_type_filter: latest.resource_type_filter,
            resource_name_filter: latest.resource_name_filter,
            pattern_type_filter: latest.pattern_type_filter,
            principal_filter: latest.principal_filter,
            host_filter: latest.host_filter,
            operation: latest.operation,
            permission_type: latest.permission_type,
        })
    }
}

impl From<DescribeAclsResponse0> for DescribeAclsResponse2 {
    fn from(older: DescribeAclsResponse0) -> Self {
        DescribeAclsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            error_message: older.error_message.into(),
            resources: older.resources.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeAclsResponseResources0> for DescribeAclsResponseResources2 {
    fn from(older: DescribeAclsResponseResources0) -> Self {
        DescribeAclsResponseResources2 {
            resource_type: older.resource_type,
            resource_name: older.resource_name.into(),
            acls: older.acls.into_iter().map(|el| el.into()).collect(),
            ..DescribeAclsResponseResources2::default()
        }
    }
}

impl From<DescribeAclsResponseResourcesAcls0> for DescribeAclsResponseResourcesAcls2 {
    fn from(older: DescribeAclsResponseResourcesAcls0) -> Self {
        DescribeAclsResponseResourcesAcls2 {
            principal: older.principal.into(),
            host: older.host.into(),
            operation: older.operation,
            permission_type: older.permission_type,
        }
    }
}

impl From<DescribeAclsResponse1> for DescribeAclsResponse2 {
    fn from(older: DescribeAclsResponse1) -> Self {
        DescribeAclsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            error_message: older.error_message.into(),
            resources: older.resources.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeAclsResponseResources1> for DescribeAclsResponseResources2 {
    fn from(older: DescribeAclsResponseResources1) -> Self {
        DescribeAclsResponseResources2 {
            resource_type: older.resource_type,
            resource_name: older.resource_name.into(),
            pattern_type: older.pattern_type,
            acls: older.acls.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeAclsResponseResourcesAcls1> for DescribeAclsResponseResourcesAcls2 {
    fn from(older: DescribeAclsResponseResourcesAcls1) -> Self {
        DescribeAclsResponseResourcesAcls2 {
            principal: older.principal.into(),
            host: older.host.into(),
            operation: older.operation,
            permission_type: older.permission_type,
        }
    }
}
