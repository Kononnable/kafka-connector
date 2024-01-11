use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct CreateAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each ACL creation.
    pub results: Vec<CreatableAclResult>,
}

#[derive(Debug, Clone)]
pub struct CreatableAclResult {
    /// The result error, or zero if there was no error.
    pub error_code: i16,

    /// The result message, or null if there was no error.
    pub error_message: String,
}

impl ApiResponse for CreateAclsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<CreatableAclResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            CreateAclsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl Default for CreateAclsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            results: Default::default(),
        }
    }
}

impl FromBytes for CreatableAclResult {
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
        CreatableAclResult {
            error_code,
            error_message,
        }
    }
}

impl Default for CreatableAclResult {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
