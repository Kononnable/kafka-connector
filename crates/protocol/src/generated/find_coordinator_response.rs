use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct FindCoordinatorResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The error message, or null if there was no error.
    pub error_message: Option<String>,

    /// The node id.
    pub node_id: i32,

    /// The host name.
    pub host: String,

    /// The port.
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
        let error_code = i16::deserialize(version, bytes);
        let error_message = if version >= 1 {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let node_id = i32::deserialize(version, bytes);
        let host = String::deserialize(version, bytes);
        let port = i32::deserialize(version, bytes);
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
