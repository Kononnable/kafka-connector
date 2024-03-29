use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteGroupsRequest {
    /// The group names to delete.
    pub groups_names: Vec<String>,
}

impl ApiRequest for DeleteGroupsRequest {
    type Response = super::delete_groups_response::DeleteGroupsResponse;

    fn get_api_key() -> i16 {
        42
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
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
        self.groups_names.serialize(version, bytes)?;
        Ok(())
    }
}

impl DeleteGroupsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.groups_names != Vec::<String>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "groups_names",
                _version,
                "DeleteGroupsRequest",
            ));
        }
        Ok(())
    }
}
