use super::prelude::*;

pub type LeaveGroupRequest = LeaveGroupRequest4;
pub type LeaveGroupResponse = LeaveGroupResponse4;
impl ApiCall for LeaveGroupRequest {
    type Response = LeaveGroupResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        4
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::LeaveGroup
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                LeaveGroupRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                LeaveGroupRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &LeaveGroupRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &LeaveGroupRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &LeaveGroupRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &LeaveGroupRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, LeaveGroupResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => LeaveGroupResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => LeaveGroupResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => LeaveGroupResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => LeaveGroupResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => LeaveGroupResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => LeaveGroupResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequest0 {
    pub group_id: String,
    pub member_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequest1 {
    pub group_id: String,
    pub member_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequest2 {
    pub group_id: String,
    pub member_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequest3 {
    pub group_id: String,
    pub members: Vec<LeaveGroupRequestMembers3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequestMembers3 {
    pub member_id: String,
    pub group_instance_id: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequest4 {
    pub group_id: String,
    pub members: Vec<LeaveGroupRequestMembers4>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequestMembers4 {
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse0 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse1 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse2 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub members: Option<Vec<LeaveGroupResponseMembers3>>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponseMembers3 {
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub members: Option<Vec<LeaveGroupResponseMembers4>>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponseMembers4 {
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<LeaveGroupRequest4> for LeaveGroupRequest0 {
    fn from(latest: LeaveGroupRequest4) -> LeaveGroupRequest0 {
        log::debug!("Using old api format - LeaveGroupRequest0, ignoring field members");
        LeaveGroupRequest0 {
            group_id: latest.group_id,
            ..LeaveGroupRequest0::default()
        }
    }
}

impl From<LeaveGroupRequest4> for LeaveGroupRequest1 {
    fn from(latest: LeaveGroupRequest4) -> LeaveGroupRequest1 {
        log::debug!("Using old api format - LeaveGroupRequest1, ignoring field members");
        LeaveGroupRequest1 {
            group_id: latest.group_id,
            ..LeaveGroupRequest1::default()
        }
    }
}

impl From<LeaveGroupRequest4> for LeaveGroupRequest2 {
    fn from(latest: LeaveGroupRequest4) -> LeaveGroupRequest2 {
        log::debug!("Using old api format - LeaveGroupRequest2, ignoring field members");
        LeaveGroupRequest2 {
            group_id: latest.group_id,
            ..LeaveGroupRequest2::default()
        }
    }
}

impl From<LeaveGroupRequest4> for LeaveGroupRequest3 {
    fn from(latest: LeaveGroupRequest4) -> LeaveGroupRequest3 {
        LeaveGroupRequest3 {
            group_id: latest.group_id,
            members: latest.members.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<LeaveGroupRequestMembers4> for LeaveGroupRequestMembers3 {
    fn from(latest: LeaveGroupRequestMembers4) -> LeaveGroupRequestMembers3 {
        LeaveGroupRequestMembers3 {
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
        }
    }
}

impl From<LeaveGroupResponse0> for LeaveGroupResponse4 {
    fn from(older: LeaveGroupResponse0) -> Self {
        LeaveGroupResponse4 {
            error_code: older.error_code,
            ..LeaveGroupResponse4::default()
        }
    }
}

impl From<LeaveGroupResponse1> for LeaveGroupResponse4 {
    fn from(older: LeaveGroupResponse1) -> Self {
        LeaveGroupResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            ..LeaveGroupResponse4::default()
        }
    }
}

impl From<LeaveGroupResponse2> for LeaveGroupResponse4 {
    fn from(older: LeaveGroupResponse2) -> Self {
        LeaveGroupResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            ..LeaveGroupResponse4::default()
        }
    }
}

impl From<LeaveGroupResponse3> for LeaveGroupResponse4 {
    fn from(older: LeaveGroupResponse3) -> Self {
        LeaveGroupResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            members: older
                .members
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            ..LeaveGroupResponse4::default()
        }
    }
}

impl From<LeaveGroupResponseMembers3> for LeaveGroupResponseMembers4 {
    fn from(older: LeaveGroupResponseMembers3) -> Self {
        LeaveGroupResponseMembers4 {
            member_id: older.member_id,
            group_instance_id: older.group_instance_id,
            error_code: older.error_code,
            ..LeaveGroupResponseMembers4::default()
        }
    }
}
