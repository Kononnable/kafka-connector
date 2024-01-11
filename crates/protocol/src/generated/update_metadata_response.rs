use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct UpdateMetadataResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for UpdateMetadataResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (header, UpdateMetadataResponse { error_code })
    }
}

impl Default for UpdateMetadataResponse {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
        }
    }
}
