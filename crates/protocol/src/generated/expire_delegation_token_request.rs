use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpireDelegationTokenRequest {
    /// The HMAC of the delegation token to be expired.
    pub hmac: Vec<u8>,

    /// The expiry time period in milliseconds.
    pub expiry_time_period_ms: i64,
}

impl ApiRequest for ExpireDelegationTokenRequest {
    type Response = super::expire_delegation_token_response::ExpireDelegationTokenResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(40)
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
        self.hmac.serialize(version, _bytes)?;
        self.expiry_time_period_ms.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let hmac = Vec::<u8>::deserialize(version, bytes);
        let expiry_time_period_ms = i64::deserialize(version, bytes);
        ExpireDelegationTokenRequest {
            hmac,
            expiry_time_period_ms,
        }
    }
}

impl ExpireDelegationTokenRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
