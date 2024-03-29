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

    fn get_api_key() -> i16 {
        39
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
        self.renew_period_ms.serialize(version, bytes)?;
        Ok(())
    }
}

impl RenewDelegationTokenRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
