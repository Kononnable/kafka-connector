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

    fn get_api_key() -> i16 {
        40
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.hmac.serialize(version, bytes)?;
        self.expiry_time_period_ms.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let hmac = Vec::<u8>::deserialize(version, bytes);
        let expiry_time_period_ms = i64::deserialize(version, bytes);
        ExpireDelegationTokenRequest {
            hmac,
            expiry_time_period_ms,
        }
    }
}

impl ExpireDelegationTokenRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
