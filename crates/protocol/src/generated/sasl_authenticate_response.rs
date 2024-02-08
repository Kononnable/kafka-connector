use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
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
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
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
