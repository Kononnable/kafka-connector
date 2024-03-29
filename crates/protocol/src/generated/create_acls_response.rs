use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreateAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each ACL creation.
    pub results: Vec<CreatableAclResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatableAclResult {
    /// The result error, or zero if there was no error.
    pub error_code: i16,

    /// The result message, or null if there was no error.
    pub error_message: Option<String>,
}

impl ApiResponse for CreateAclsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<CreatableAclResult>::deserialize(version, bytes);
        (
            header,
            CreateAclsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for CreatableAclResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        CreatableAclResult {
            error_code,
            error_message,
        }
    }
}
