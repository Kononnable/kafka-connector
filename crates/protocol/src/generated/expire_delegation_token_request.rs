use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ExpireDelegationTokenRequest {
    pub hmac: Vec<u8>,
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
            self.expiry_time_period_ms.serialize(version, bytes);
        }
    }
}
