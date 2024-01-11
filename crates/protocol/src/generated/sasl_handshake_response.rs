use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct SaslHandshakeResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The mechanisms enabled in the server.
    pub mechanisms: Vec<String>,
}

impl ApiResponse for SaslHandshakeResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let mechanisms = if version >= 0 {
            Vec::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            SaslHandshakeResponse {
                error_code,
                mechanisms,
            },
        )
    }
}

impl Default for SaslHandshakeResponse {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            mechanisms: Default::default(),
        }
    }
}
