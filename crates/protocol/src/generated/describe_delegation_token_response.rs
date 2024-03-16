use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The tokens.
    pub tokens: Vec<DescribedDelegationToken>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribedDelegationToken {
    /// The token principal type.
    pub principal_type: String,

    /// The token principal name.
    pub principal_name: String,

    /// The token issue timestamp in milliseconds.
    pub issue_timestamp: i64,

    /// The token expiry timestamp in milliseconds.
    pub expiry_timestamp: i64,

    /// The token maximum timestamp length in milliseconds.
    pub max_timestamp: i64,

    /// The token ID.
    pub token_id: String,

    /// The token HMAC.
    pub hmac: Vec<u8>,

    /// Those who are able to renew this token before it expires.
    pub renewers: Vec<DescribedDelegationTokenRenewer>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribedDelegationTokenRenewer {
    /// The renewer principal type
    pub principal_type: String,

    /// The renewer principal name
    pub principal_name: String,
}

impl ApiResponse for DescribeDelegationTokenResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let tokens = Vec::<DescribedDelegationToken>::deserialize(version, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        (
            header,
            DescribeDelegationTokenResponse {
                error_code,
                tokens,
                throttle_time_ms,
            },
        )
    }
}

impl FromBytes for DescribedDelegationToken {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let principal_type = String::deserialize(version, bytes);
        let principal_name = String::deserialize(version, bytes);
        let issue_timestamp = i64::deserialize(version, bytes);
        let expiry_timestamp = i64::deserialize(version, bytes);
        let max_timestamp = i64::deserialize(version, bytes);
        let token_id = String::deserialize(version, bytes);
        let hmac = Vec::<u8>::deserialize(version, bytes);
        let renewers = Vec::<DescribedDelegationTokenRenewer>::deserialize(version, bytes);
        DescribedDelegationToken {
            principal_type,
            principal_name,
            issue_timestamp,
            expiry_timestamp,
            max_timestamp,
            token_id,
            hmac,
            renewers,
        }
    }
}

impl FromBytes for DescribedDelegationTokenRenewer {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let principal_type = String::deserialize(version, bytes);
        let principal_name = String::deserialize(version, bytes);
        DescribedDelegationTokenRenewer {
            principal_type,
            principal_name,
        }
    }
}
