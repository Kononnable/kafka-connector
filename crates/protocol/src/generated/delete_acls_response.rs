use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each filter.
    pub filter_results: Vec<DeleteAclsFilterResult>,
}

#[derive(Clone, Debug, Default)]
pub struct DeleteAclsFilterResult {
    /// The error code, or 0 if the filter succeeded.
    pub error_code: i16,

    /// The error message, or null if the filter succeeded.
    pub error_message: Option<String>,

    /// The ACLs which matched this filter.
    pub matching_acls: Vec<DeleteAclsMatchingAcl>,
}

#[derive(Clone, Debug)]
pub struct DeleteAclsMatchingAcl {
    /// The deletion error code, or 0 if the deletion succeeded.
    pub error_code: i16,

    /// The deletion error message, or null if the deletion succeeded.
    pub error_message: Option<String>,

    /// The ACL resource type.
    pub resource_type: i8,

    /// The ACL resource name.
    pub resource_name: String,

    /// The ACL resource pattern type.
    pub pattern_type: i8,

    /// The ACL principal.
    pub principal: String,

    /// The ACL host.
    pub host: String,

    /// The ACL operation.
    pub operation: i8,

    /// The ACL permission type.
    pub permission_type: i8,
}

impl ApiResponse for DeleteAclsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let filter_results = if version >= 0 {
            Vec::<DeleteAclsFilterResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            DeleteAclsResponse {
                throttle_time_ms,
                filter_results,
            },
        )
    }
}

impl FromBytes for DeleteAclsFilterResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_message = if version >= 0 {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let matching_acls = if version >= 0 {
            Vec::<DeleteAclsMatchingAcl>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DeleteAclsFilterResult {
            error_code,
            error_message,
            matching_acls,
        }
    }
}

impl FromBytes for DeleteAclsMatchingAcl {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_message = if version >= 0 {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resource_type = if version >= 0 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resource_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let pattern_type = if version >= 1 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
        DeleteAclsMatchingAcl {
            error_code,
            error_message,
            resource_type,
            resource_name,
            pattern_type,
            principal,
            host,
            operation,
            permission_type,
        }
    }
}

impl Default for DeleteAclsMatchingAcl {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            error_message: Default::default(),
            resource_type: Default::default(),
            resource_name: Default::default(),
            pattern_type: 3,
            principal: Default::default(),
            host: Default::default(),
            operation: Default::default(),
            permission_type: Default::default(),
        }
    }
}
