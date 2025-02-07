use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenewDelegationTokenRequest {
    /// The HMAC of the delegation token to be renewed.
    pub hmac: Vec<u8>,

    /// The renewal time period in milliseconds.
    pub renew_period_ms: i64,
}

impl ApiRequest for RenewDelegationTokenRequest {
    type Response = super::renew_delegation_token_response::RenewDelegationTokenResponse;

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
        self.hmac.serialize(version, _bytes);
        self.renew_period_ms.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let hmac = Vec::<u8>::deserialize(version, bytes);
        let renew_period_ms = i64::deserialize(version, bytes);
        RenewDelegationTokenRequest {
            hmac,
            renew_period_ms,
        }
    }
}

impl RenewDelegationTokenRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
