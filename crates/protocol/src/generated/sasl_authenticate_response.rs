use super::super::prelude::*;

/// Version 1 adds the session lifetime.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SaslAuthenticateResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The error message, or null if there was no error.
    pub error_message: Option<String>,

    /// The SASL authentication bytes from the server, as defined by the SASL mechanism.
    pub auth_bytes: Vec<u8>,

    /// The SASL authentication bytes from the server, as defined by the SASL mechanism.
    pub session_lifetime_ms: i64,
}

impl ApiResponse for SaslAuthenticateResponse {
    type Request = super::sasl_authenticate_request::SaslAuthenticateRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(36)
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
        self.error_message.serialize(version, _bytes);
        self.auth_bytes.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.session_lifetime_ms.serialize(version, _bytes);
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let auth_bytes = Vec::<u8>::deserialize(version, bytes);
        let session_lifetime_ms = if version >= ApiVersion(1) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        SaslAuthenticateResponse {
            error_code,
            error_message,
            auth_bytes,
            session_lifetime_ms,
        }
    }
}

impl SaslAuthenticateResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.session_lifetime_ms != 0 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "session_lifetime_ms",
                *_version,
                "SaslAuthenticateResponse",
            ));
        }
        Ok(())
    }
}
