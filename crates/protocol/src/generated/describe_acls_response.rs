use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The error message, or null if there was no error.
    pub error_message: String,

    /// Each Resource that is referenced in an ACL.
    pub resources: Vec<DescribeAclsResource>,
}

#[derive(Clone, Debug)]
pub struct DescribeAclsResource {
    /// The resource type.
    pub r#type: i8,

    /// The resource name.
    pub name: String,

    /// The resource pattern type.
    pub pattern_type: i8,

    /// The ACLs.
    pub acls: Vec<AclDescription>,
}

#[derive(Clone, Debug, Default)]
pub struct AclDescription {
    /// The ACL principal.
    pub principal: String,

    /// The ACL host.
    pub host: String,

    /// The ACL operation.
    pub operation: i8,

    /// The ACL permission type.
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

impl Default for DescribeAclsResource {
    fn default() -> Self {
        Self {
            r#type: Default::default(),
            name: Default::default(),
            pattern_type: 3,
            acls: Default::default(),
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
