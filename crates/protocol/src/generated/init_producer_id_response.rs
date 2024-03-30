use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
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
    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let producer_id = i64::deserialize(version, bytes);
        let producer_epoch = i16::deserialize(version, bytes);
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
