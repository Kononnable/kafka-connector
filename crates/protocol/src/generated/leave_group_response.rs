use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct LeaveGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for LeaveGroupResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            LeaveGroupResponse {
                throttle_time_ms,
                error_code,
            },
        )
    }
}

impl Default for LeaveGroupResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            error_code: Default::default(),
        }
    }
}
