use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct JoinGroupRequest {
    /// The group identifier.
    pub group_id: String,

    /// The coordinator considers the consumer dead if it receives no heartbeat after this timeout in milliseconds.
    pub session_timeout_ms: i32,

    /// The maximum time in milliseconds that the coordinator will wait for each member to rejoin when rebalancing the group.
    pub rebalance_timeout_ms: i32,

    /// The member id assigned by the group coordinator.
    pub member_id: String,

    /// The unique name the for class of protocols implemented by the group we want to join.
    pub protocol_type: String,

    /// The list of protocols that the member supports.
    pub protocols: Vec<JoinGroupRequestProtocol>,
}

#[derive(Debug, Clone)]
pub struct JoinGroupRequestProtocol {
    /// The protocol name.
    pub name: String,

    /// The protocol metadata.
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

impl Default for JoinGroupRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            session_timeout_ms: Default::default(),
            rebalance_timeout_ms: -1,
            member_id: Default::default(),
            protocol_type: Default::default(),
            protocols: Default::default(),
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

impl Default for JoinGroupRequestProtocol {
    fn default() -> Self {
        Self {
            name: Default::default(),
            metadata: Default::default(),
        }
    }
}
