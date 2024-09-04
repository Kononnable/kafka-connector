use super::super::prelude::*;

/// Versions 0 through 2 of ApiVersionsRequest are the same.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApiVersionsRequest {}

impl ApiRequest for ApiVersionsRequest {
    type Response = super::api_versions_response::ApiVersionsResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(18)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(2)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        Ok(())
    }

    fn deserialize(_version: ApiVersion, _bytes: &mut BytesMut) -> Self {
        ApiVersionsRequest {}
    }
}

impl ApiVersionsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
