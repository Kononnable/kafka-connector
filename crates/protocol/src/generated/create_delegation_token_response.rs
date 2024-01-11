use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct CreateDelegationTokenResponse {
    /// The top-level error, or zero if there was no error.
    pub error_code: i16,

    /// The principal type of the token owner.
    pub principal_type: String,

    /// The name of the token owner.
    pub principal_name: String,

    /// When this token was generated.
    pub issue_timestamp_ms: i64,

    /// When this token expires.
    pub expiry_timestamp_ms: i64,

    /// The maximum lifetime of this token.
    pub max_timestamp_ms: i64,

    /// The token UUID.
    pub token_id: String,

    /// HMAC of the delegation token.
    pub hmac: Vec<u8>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
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

impl Default for CreateDelegationTokenResponse {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            principal_type: Default::default(),
            principal_name: Default::default(),
            issue_timestamp_ms: Default::default(),
            expiry_timestamp_ms: Default::default(),
            max_timestamp_ms: Default::default(),
            token_id: Default::default(),
            hmac: Default::default(),
            throttle_time_ms: Default::default(),
        }
    }
}
