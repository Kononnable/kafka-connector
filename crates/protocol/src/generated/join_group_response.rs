use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct JoinGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The generation ID of the group.
    pub generation_id: i32,

    /// The group protocol selected by the coordinator.
    pub protocol_name: String,

    /// The leader of the group.
    pub leader: String,

    /// The member ID assigned by the group coordinator.
    pub member_id: String,

    pub members: Vec<JoinGroupResponseMember>,
}

#[derive(Debug, Clone)]
pub struct JoinGroupResponseMember {
    /// The group member ID.
    pub member_id: String,

    /// The group member metadata.
    pub metadata: Vec<u8>,
}

impl ApiResponse for JoinGroupResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 2 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let generation_id = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let protocol_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let member_id = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let members = if version >= 0 {
            Vec::<JoinGroupResponseMember>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            JoinGroupResponse {
                throttle_time_ms,
                error_code,
                generation_id,
                protocol_name,
                leader,
                member_id,
                members,
            },
        )
    }
}

impl Default for JoinGroupResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            error_code: Default::default(),
            generation_id: Default::default(),
            protocol_name: Default::default(),
            leader: Default::default(),
            member_id: Default::default(),
            members: Default::default(),
        }
    }
}

impl FromBytes for JoinGroupResponseMember {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let member_id = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let metadata = if version >= 0 {
            Vec::<u8>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        JoinGroupResponseMember {
            member_id,
            metadata,
        }
    }
}

impl Default for JoinGroupResponseMember {
    fn default() -> Self {
        Self {
            member_id: Default::default(),
            metadata: Default::default(),
        }
    }
}
