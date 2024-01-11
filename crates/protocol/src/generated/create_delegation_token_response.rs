use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreateDelegationTokenResponse {
    pub error_code: i16,
    pub principal_type: String,
    pub principal_name: String,
    pub issue_timestamp_ms: i64,
    pub expiry_timestamp_ms: i64,
    pub max_timestamp_ms: i64,
    pub token_id: String,
    pub hmac: Vec<u8>,
    pub throttle_time_ms: i32,
}

impl ApiResponse for CreateDelegationTokenResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let principal_type = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let principal_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let issue_timestamp_ms = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let expiry_timestamp_ms = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let max_timestamp_ms = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let token_id = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let hmac = if version >= 0 {
            Vec::<u8>::deserialize(version, bytes)
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
            CreateDelegationTokenResponse {
                error_code,
                principal_type,
                principal_name,
                issue_timestamp_ms,
                expiry_timestamp_ms,
                max_timestamp_ms,
                token_id,
                hmac,
                throttle_time_ms,
            },
        )
    }
}
