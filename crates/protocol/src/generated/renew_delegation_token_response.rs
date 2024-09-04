use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenewDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The timestamp in milliseconds at which this token expires.
    pub expiry_timestamp_ms: i64,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

impl ApiResponse for RenewDelegationTokenResponse {
    type Request = super::renew_delegation_token_request::RenewDelegationTokenRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(39)
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
        self.error_code.serialize(version, _bytes)?;
        self.expiry_timestamp_ms.serialize(version, _bytes)?;
        self.throttle_time_ms.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let expiry_timestamp_ms = i64::deserialize(version, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        RenewDelegationTokenResponse {
            error_code,
            expiry_timestamp_ms,
            throttle_time_ms,
        }
    }
}

impl RenewDelegationTokenResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
