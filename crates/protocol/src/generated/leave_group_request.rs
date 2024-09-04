use super::super::prelude::*;

/// Version 1 and 2 are the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeaveGroupRequest {
    /// The ID of the group to leave.
    pub group_id: String,

    /// The member ID to remove from the group.
    pub member_id: String,
}

impl ApiRequest for LeaveGroupRequest {
    type Response = super::leave_group_response::LeaveGroupResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(13)
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
        self.group_id.serialize(version, _bytes)?;
        self.member_id.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let member_id = String::deserialize(version, bytes);
        LeaveGroupRequest {
            group_id,
            member_id,
        }
    }
}

impl LeaveGroupRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
