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

    fn get_api_key() -> ApiKey {
        ApiKey(17)
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
        self.error_code.serialize(version, _bytes);
        self.mechanisms.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let mechanisms = Vec::<String>::deserialize(version, bytes);
        SaslHandshakeResponse {
            error_code,
            mechanisms,
        }
    }
}

impl SaslHandshakeResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
