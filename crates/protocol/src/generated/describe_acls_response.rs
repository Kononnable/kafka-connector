use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeAclsResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub error_message: String,
    pub resources: Vec<DescribeAclsResource>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribeAclsResource {
    pub r#type: i8,
    pub name: String,
    pub pattern_type: i8,
    pub acls: Vec<AclDescription>,
}

#[derive(Debug, Default, Clone)]
pub struct AclDescription {
    pub principal: String,
    pub host: String,
    pub operation: i8,
    pub permission_type: i8,
}

impl ApiResponse for DescribeAclsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_message = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resources = if version >= 0 {
            Vec::<DescribeAclsResource>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            DescribeAclsResponse {
                throttle_time_ms,
                error_code,
                error_message,
                resources,
            },
        )
    }
}

impl FromBytes for DescribeAclsResource {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let r#type = if version >= 0 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let pattern_type = if version >= 1 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let acls = if version >= 0 {
            Vec::<AclDescription>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribeAclsResource {
            r#type,
            name,
            pattern_type,
            acls,
        }
    }
}

impl FromBytes for AclDescription {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let principal = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let host = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let operation = if version >= 0 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let permission_type = if version >= 0 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        AclDescription {
            principal,
            host,
            operation,
            permission_type,
        }
    }
}
