use super::super::prelude::*;

/// Versions 1 and 2 are the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeGroupsRequest {
    /// The names of the groups to describe
    pub groups: Vec<String>,
}

impl ApiRequest for DescribeGroupsRequest {
    type Response = super::describe_groups_response::DescribeGroupsResponse;

    fn get_api_key() -> i16 {
        15
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
        self.groups.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribeGroupsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.groups != Vec::<String>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "groups",
                _version,
                "DescribeGroupsRequest",
            ));
        }
        Ok(())
    }
}
