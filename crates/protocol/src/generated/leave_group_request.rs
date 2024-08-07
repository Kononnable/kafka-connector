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

    fn get_api_key() -> i16 {
        13
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
        self.group_id.serialize(version, bytes)?;
        self.member_id.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let member_id = String::deserialize(version, bytes);
        LeaveGroupRequest {
            group_id,
            member_id,
        }
    }
}

impl LeaveGroupRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
