use super::super::prelude::*;

/// Version 1 supports SASL_AUTHENTICATE.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SaslHandshakeRequest {
    /// The SASL mechanism chosen by the client.
    pub mechanism: String,
}

impl ApiRequest for SaslHandshakeRequest {
    type Response = super::sasl_handshake_response::SaslHandshakeResponse;

    fn get_api_key() -> i16 {
        17
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
        self.mechanism.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let mechanism = String::deserialize(version, bytes);
        SaslHandshakeRequest { mechanism }
    }
}

impl SaslHandshakeRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
