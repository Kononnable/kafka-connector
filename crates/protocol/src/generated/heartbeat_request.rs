use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct HeartbeatRequest {
    /// The group id.
    pub group_id: String,

    /// The generation of the group.
    pub generationid: i32,

    /// The member ID.
    pub member_id: String,
}

impl ApiRequest for HeartbeatRequest {
    type Response = super::heartbeat_response::HeartbeatResponse;

    fn get_api_key() -> i16 {
        12
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
            self.generationid.serialize(version, bytes);
        }
        if version >= 0 {
            self.member_id.serialize(version, bytes);
        }
    }
}

impl Default for HeartbeatRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            generationid: Default::default(),
            member_id: Default::default(),
        }
    }
}
