use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
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
    type Request = super::create_delegation_token_request::CreateDelegationTokenRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(38)
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
        self.principal_type.serialize(version, _bytes);
        self.principal_name.serialize(version, _bytes);
        self.issue_timestamp_ms.serialize(version, _bytes);
        self.expiry_timestamp_ms.serialize(version, _bytes);
        self.max_timestamp_ms.serialize(version, _bytes);
        self.token_id.serialize(version, _bytes);
        self.hmac.serialize(version, _bytes);
        self.throttle_time_ms.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let principal_type = String::deserialize(version, bytes);
        let principal_name = String::deserialize(version, bytes);
        let issue_timestamp_ms = i64::deserialize(version, bytes);
        let expiry_timestamp_ms = i64::deserialize(version, bytes);
        let max_timestamp_ms = i64::deserialize(version, bytes);
        let token_id = String::deserialize(version, bytes);
        let hmac = Vec::<u8>::deserialize(version, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
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
        }
    }
}

impl CreateDelegationTokenResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
