use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct JoinGroupRequest {
    pub group_id: String,
    pub session_timeout_ms: i32,
    pub rebalance_timeout_ms: i32,
    pub member_id: String,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocol>,
}

#[derive(Debug, Default, Clone)]
pub struct JoinGroupRequestProtocol {
    pub name: String,
    pub metadata: Vec<u8>,
}

impl ApiRequest for JoinGroupRequest {
    type Response = super::join_group_response::JoinGroupResponse;

    fn get_api_key() -> i16 {
        11
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        4
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
            self.session_timeout_ms.serialize(version, bytes);
        }
        if version >= 1 {
            self.rebalance_timeout_ms.serialize(version, bytes);
        }
        if version >= 0 {
            self.member_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.protocol_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.protocols.serialize(version, bytes);
        }
    }
}

impl ToBytes for JoinGroupRequestProtocol {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.metadata.serialize(version, bytes);
        }
    }
}
