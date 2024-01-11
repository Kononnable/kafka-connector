use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct InitProducerIdResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The current producer id.
    pub producer_id: i64,

    /// The current epoch associated with the producer id.
    pub producer_epoch: i16,
}

impl ApiResponse for InitProducerIdResponse {
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
        let producer_id = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let producer_epoch = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            InitProducerIdResponse {
                throttle_time_ms,
                error_code,
                producer_id,
                producer_epoch,
            },
        )
    }
}

impl Default for InitProducerIdResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            error_code: Default::default(),
            producer_id: Default::default(),
            producer_epoch: Default::default(),
        }
    }
}
