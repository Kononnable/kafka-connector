use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteGroupsRequest {
    /// The group names to delete.
    pub groups_names: Vec<String>,
}

impl ApiRequest for DeleteGroupsRequest {
    type Response = super::delete_groups_response::DeleteGroupsResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(42)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(1)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.groups_names.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let groups_names = Vec::<String>::deserialize(version, bytes);
        DeleteGroupsRequest { groups_names }
    }
}

impl DeleteGroupsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
