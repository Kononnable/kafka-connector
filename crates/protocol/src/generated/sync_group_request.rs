use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.group_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.generation_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.member_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.assignments.serialize(version, bytes);
        }
    }
}

impl ToBytes for SyncGroupRequestAssignment {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.member_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.assignment.serialize(version, bytes);
        }
    }
}
