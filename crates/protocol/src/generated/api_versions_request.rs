use super::super::prelude::*;

/// Versions 0 through 2 of ApiVersionsRequest are the same.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApiVersionsRequest {}

impl ApiRequest for ApiVersionsRequest {
    type Response = super::api_versions_response::ApiVersionsResponse;

    fn get_api_key() -> i16 {
        18
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        Ok(())
    }

    fn deserialize(_version: i16, _bytes: &mut BytesMut) -> Self {
        ApiVersionsRequest {}
    }
}

impl ApiVersionsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
