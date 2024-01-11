use super::super::prelude::*;

#[derive(Clone, Debug)]
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.hmac.serialize(version, bytes);
        }
        if version >= 0 {
            self.renew_period_ms.serialize(version, bytes);
        }
    }
}

impl Default for RenewDelegationTokenRequest {
    fn default() -> Self {
        Self {
            hmac: Default::default(),
            renew_period_ms: Default::default(),
        }
    }
}
