use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteAclsResponse {
    pub throttle_time_ms: i32,
    pub filter_results: Vec<DeleteAclsFilterResult>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteAclsFilterResult {
    pub error_code: i16,
    pub error_message: String,
    pub matching_acls: Vec<DeleteAclsMatchingAcl>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteAclsMatchingAcl {
    pub error_code: i16,
    pub error_message: String,
    pub resource_type: i8,
    pub resource_name: String,
    pub pattern_type: i8,
    pub principal: String,
    pub host: String,
    pub operation: i8,
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
            String::deserialize(version, bytes)
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
            String::deserialize(version, bytes)
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
