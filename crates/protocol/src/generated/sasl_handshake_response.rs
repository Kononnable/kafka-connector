use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct SaslHandshakeResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The mechanisms enabled in the server.
    pub mechanisms: Vec<String>,
}

impl ApiResponse for SaslHandshakeResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
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
