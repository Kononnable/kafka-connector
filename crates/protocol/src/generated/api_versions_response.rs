use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ApiVersionsResponse {
    /// The top-level error code.
    pub error_code: i16,

    /// The APIs supported by the broker.
    pub api_keys: Vec<ApiVersionsResponseKey>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ApiVersionsResponseKey {
    /// The API index.
    pub index: i16,

    /// The minimum supported version, inclusive.
    pub min_version: i16,

    /// The maximum supported version, inclusive.
    pub max_version: i16,
}

impl ApiResponse for ApiVersionsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let api_keys = if version >= 0 {
            Vec::<ApiVersionsResponseKey>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            ApiVersionsResponse {
                error_code,
                api_keys,
                throttle_time_ms,
            },
        )
    }
}

impl FromBytes for ApiVersionsResponseKey {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let index = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let min_version = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let max_version = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ApiVersionsResponseKey {
            index,
            min_version,
            max_version,
        }
    }
}
