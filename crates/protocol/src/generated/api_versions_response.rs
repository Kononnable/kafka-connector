use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ApiVersionsResponse {
    pub error_code: i16,
    pub api_keys: Vec<ApiVersionsResponseKey>,
    pub throttle_time_ms: i32,
}

#[derive(Debug, Default, Clone)]
pub struct ApiVersionsResponseKey {
    pub index: i16,
    pub min_version: i16,
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
