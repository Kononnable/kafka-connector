use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct RenewDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The timestamp in milliseconds at which this token expires.
    pub expiry_timestamp_ms: i64,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

impl ApiResponse for RenewDelegationTokenResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let expiry_timestamp_ms = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            RenewDelegationTokenResponse {
                error_code,
                expiry_timestamp_ms,
                throttle_time_ms,
            },
        )
    }
}

impl Default for RenewDelegationTokenResponse {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            expiry_timestamp_ms: Default::default(),
            throttle_time_ms: Default::default(),
        }
    }
}