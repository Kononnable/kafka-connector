use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreateDelegationTokenRequest {
    pub renewers: Vec<CreatableRenewers>,
    pub max_lifetime_ms: i64,
}

#[derive(Debug, Default, Clone)]
pub struct CreatableRenewers {
    pub principal_type: String,
    pub principal_name: String,
}

impl ApiRequest for CreateDelegationTokenRequest {
    type Response = super::create_delegation_token_response::CreateDelegationTokenResponse;

    fn get_api_key() -> i16 {
        38
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
            self.renewers.serialize(version, bytes);
        }
        if version >= 0 {
            self.max_lifetime_ms.serialize(version, bytes);
        }
    }
}

impl ToBytes for CreatableRenewers {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.principal_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.principal_name.serialize(version, bytes);
        }
    }
}
