use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SaslHandshakeResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The mechanisms enabled in the server.
    pub mechanisms: Vec<String>,
}

impl ApiResponse for SaslHandshakeResponse {
    type Request = super::sasl_handshake_request::SaslHandshakeRequest;

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
        header: &ResponseHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.error_code.serialize(version, bytes)?;
        self.mechanisms.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let mechanisms = Vec::<String>::deserialize(version, bytes);
        (
            header,
            SaslHandshakeResponse {
                error_code,
                mechanisms,
            },
        )
    }
}

impl SaslHandshakeResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
