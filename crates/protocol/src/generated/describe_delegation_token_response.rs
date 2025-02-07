use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
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
    type Request = super::describe_delegation_token_request::DescribeDelegationTokenRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(41)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(1)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes);
        self.tokens.serialize(version, _bytes);
        self.throttle_time_ms.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let tokens = Vec::<DescribedDelegationToken>::deserialize(version, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        DescribeDelegationTokenResponse {
            error_code,
            tokens,
            throttle_time_ms,
        }
    }
}

impl DescribeDelegationTokenResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.tokens.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for DescribedDelegationToken {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.principal_type.serialize(version, _bytes);
        self.principal_name.serialize(version, _bytes);
        self.issue_timestamp.serialize(version, _bytes);
        self.expiry_timestamp.serialize(version, _bytes);
        self.max_timestamp.serialize(version, _bytes);
        self.token_id.serialize(version, _bytes);
        self.hmac.serialize(version, _bytes);
        self.renewers.serialize(version, _bytes);
    }
}

impl DescribedDelegationToken {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.renewers.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for DescribedDelegationToken {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
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

impl ToBytes for DescribedDelegationTokenRenewer {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.principal_type.serialize(version, _bytes);
        self.principal_name.serialize(version, _bytes);
    }
}

impl DescribedDelegationTokenRenewer {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribedDelegationTokenRenewer {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let principal_type = String::deserialize(version, bytes);
        let principal_name = String::deserialize(version, bytes);
        DescribedDelegationTokenRenewer {
            principal_type,
            principal_name,
        }
    }
}
