use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct SyncGroupRequest {
    pub group_id: String,
    pub generation_id: i32,
    pub member_id: String,
    pub assignments: Vec<SyncGroupRequestAssignment>,
}

#[derive(Debug, Default, Clone)]
pub struct SyncGroupRequestAssignment {
    pub member_id: String,
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
