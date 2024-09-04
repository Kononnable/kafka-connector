use super::super::prelude::*;

/// Version 1 and 2 are the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListGroupsRequest {}

impl ApiRequest for ListGroupsRequest {
    type Response = super::list_groups_response::ListGroupsResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(16)
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
        ListGroupsRequest {}
    }
}

impl ListGroupsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
