use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct FindCoordinatorResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub error_message: String,
    pub node_id: i32,
    pub host: String,
    pub port: i32,
}

impl ApiResponse for FindCoordinatorResponse {
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
        let error_message = if version >= 1 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let node_id = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let host = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let port = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            FindCoordinatorResponse {
                throttle_time_ms,
                error_code,
                error_message,
                node_id,
                host,
                port,
            },
        )
    }
}
