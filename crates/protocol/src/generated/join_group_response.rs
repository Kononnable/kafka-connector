use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct JoinGroupResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub generation_id: i32,
    pub protocol_name: String,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMember>,
}

#[derive(Debug, Default, Clone)]
pub struct JoinGroupResponseMember {
    pub member_id: String,
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
