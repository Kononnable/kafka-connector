use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct InitProducerIdResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub producer_id: i64,
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
