use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DescribeDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The tokens.
    pub tokens: Vec<DescribedDelegationToken>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct DescribedDelegationTokenRenewer {
    /// The renewer principal type
    pub principal_type: String,

    /// The renewer principal name
    pub principal_name: String,
}

impl ApiResponse for DescribeDelegationTokenResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let tokens = if version >= 0 {
            Vec::<DescribedDelegationToken>::deserialize(version, bytes)
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
            DescribeDelegationTokenResponse {
                error_code,
                tokens,
                throttle_time_ms,
            },
        )
    }
}

impl Default for DescribeDelegationTokenResponse {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            tokens: Default::default(),
            throttle_time_ms: Default::default(),
        }
    }
}

impl FromBytes for DescribedDelegationToken {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
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
        let issue_timestamp = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let expiry_timestamp = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let max_timestamp = if version >= 0 {
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
        let renewers = if version >= 0 {
            Vec::<DescribedDelegationTokenRenewer>::deserialize(version, bytes)
        } else {
            Default::default()
        };
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

impl Default for DescribedDelegationToken {
    fn default() -> Self {
        Self {
            principal_type: Default::default(),
            principal_name: Default::default(),
            issue_timestamp: Default::default(),
            expiry_timestamp: Default::default(),
            max_timestamp: Default::default(),
            token_id: Default::default(),
            hmac: Default::default(),
            renewers: Default::default(),
        }
    }
}

impl FromBytes for DescribedDelegationTokenRenewer {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
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
        DescribedDelegationTokenRenewer {
            principal_type,
            principal_name,
        }
    }
}

impl Default for DescribedDelegationTokenRenewer {
    fn default() -> Self {
        Self {
            principal_type: Default::default(),
            principal_name: Default::default(),
        }
    }
}
