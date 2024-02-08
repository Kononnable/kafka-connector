use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct EndTxnResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for EndTxnResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        (
            header,
            EndTxnResponse {
                throttle_time_ms,
                error_code,
            },
        )
    }
}
