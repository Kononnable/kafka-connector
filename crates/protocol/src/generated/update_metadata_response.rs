use super::super::prelude::*;

/// Versions 1, 2, 3, 4, and 5 are the same as version 0
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpdateMetadataResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for UpdateMetadataResponse {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        (header, UpdateMetadataResponse { error_code })
    }
}
