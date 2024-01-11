use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct EndTxnResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
}

impl ApiResponse for EndTxnResponse {
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
        (
            header,
            EndTxnResponse {
                throttle_time_ms,
                error_code,
            },
        )
    }
}
