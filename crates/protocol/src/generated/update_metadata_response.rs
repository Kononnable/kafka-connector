use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct UpdateMetadataResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for UpdateMetadataResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        (header, UpdateMetadataResponse { error_code })
    }
}
