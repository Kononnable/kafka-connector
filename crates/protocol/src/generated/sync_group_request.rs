use super::super::prelude::*;

/// Versions 1 and 2 are the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SyncGroupRequest {
    /// The unique group identifier.
    pub group_id: String,

    /// The generation of the group.
    pub generation_id: i32,

    /// The member ID assigned by the group.
    pub member_id: String,

    /// Each assignment.
    pub assignments: Vec<SyncGroupRequestAssignment>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SyncGroupRequestAssignment {
    /// The ID of the member to assign.
    pub member_id: String,

    /// The member assignment.
    pub assignment: Vec<u8>,
}

impl ApiRequest for SyncGroupRequest {
    type Response = super::sync_group_response::SyncGroupResponse;

    fn get_api_key() -> i16 {
        14
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
        self.generation_id.serialize(version, bytes)?;
        self.member_id.serialize(version, bytes)?;
        self.assignments.serialize(version, bytes)?;
        Ok(())
    }
}

impl SyncGroupRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for SyncGroupRequestAssignment {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.member_id.serialize(version, bytes)?;
        self.assignment.serialize(version, bytes)?;
        Ok(())
    }
}

impl SyncGroupRequestAssignment {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
