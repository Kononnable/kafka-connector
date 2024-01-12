use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct SaslAuthenticateRequest {
    /// The SASL authentication bytes from the client, as defined by the SASL mechanism.
    pub auth_bytes: Vec<u8>,
}

impl ApiRequest for SaslAuthenticateRequest {
    type Response = super::sasl_authenticate_response::SaslAuthenticateResponse;

    fn get_api_key() -> i16 {
        36
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
            self.auth_bytes.serialize(version, bytes);
        }
    }
}
