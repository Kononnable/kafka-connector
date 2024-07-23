use super::super::prelude::*;

/// Versions 1, 2, 3, 4, and 5 are the same as version 0
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpdateMetadataResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for UpdateMetadataResponse {
    type Request = super::update_metadata_request::UpdateMetadataRequest;

    fn get_api_key() -> i16 {
        6
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        5
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
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        (header, UpdateMetadataResponse { error_code })
    }
}

impl UpdateMetadataResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
