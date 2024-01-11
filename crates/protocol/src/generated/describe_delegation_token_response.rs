use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeDelegationTokenResponse {
    pub error_code: i16,
    pub tokens: Vec<DescribedDelegationToken>,
    pub throttle_time_ms: i32,
}

#[derive(Debug, Default, Clone)]
pub struct DescribedDelegationToken {
    pub principal_type: String,
    pub principal_name: String,
    pub issue_timestamp: i64,
    pub expiry_timestamp: i64,
    pub max_timestamp: i64,
    pub token_id: String,
    pub hmac: Vec<u8>,
    pub renewers: Vec<DescribedDelegationTokenRenewer>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribedDelegationTokenRenewer {
    pub principal_type: String,
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
