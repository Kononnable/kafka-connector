use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ExpireDelegationTokenResponse {
    pub error_code: i16,
    pub expiry_timestamp_ms: i64,
    pub throttle_time_ms: i32,
}

impl ApiResponse for ExpireDelegationTokenResponse {
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
            ExpireDelegationTokenResponse {
                error_code,
                expiry_timestamp_ms,
                throttle_time_ms,
            },
        )
    }
}
