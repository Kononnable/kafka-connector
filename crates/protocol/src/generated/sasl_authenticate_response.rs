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

    fn get_api_key() -> i16 {
        36
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
        self.error_message.serialize(version, bytes)?;
        self.auth_bytes.serialize(version, bytes)?;
        if version >= 1 {
            self.session_lifetime_ms.serialize(version, bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let auth_bytes = Vec::<u8>::deserialize(version, bytes);
        let session_lifetime_ms = if version >= 1 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            SaslAuthenticateResponse {
                error_code,
                error_message,
                auth_bytes,
                session_lifetime_ms,
            },
        )
    }
}

impl SaslAuthenticateResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.error_message.is_none() {
            return Err(SerializationError::NullValue(
                "error_message",
                _version,
                "SaslAuthenticateResponse",
            ));
        }
        if self.session_lifetime_ms != i64::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "session_lifetime_ms",
                _version,
                "SaslAuthenticateResponse",
            ));
        }
        Ok(())
    }
}
