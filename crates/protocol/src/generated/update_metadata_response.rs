use super::super::prelude::*;

/// Versions 1, 2, 3, 4, and 5 are the same as version 0
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpdateMetadataResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for UpdateMetadataResponse {
    type Request = super::update_metadata_request::UpdateMetadataRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(6)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(5)
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
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        UpdateMetadataResponse { error_code }
    }
}

impl UpdateMetadataResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
