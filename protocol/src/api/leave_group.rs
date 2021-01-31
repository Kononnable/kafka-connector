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
    fn serialize(
        self,
        version: i16,
        buf: &mut BytesMut,
        correlation_id: i32,
        client_id: &str,
    ) -> Result<(), Error> {
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
                &LeaveGroupRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &LeaveGroupRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &LeaveGroupRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &LeaveGroupRequest3::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, LeaveGroupResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => LeaveGroupResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => LeaveGroupResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => LeaveGroupResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => LeaveGroupResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => LeaveGroupResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => LeaveGroupResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
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
    pub members: Optional<Vec<LeaveGroupRequestMembers3>>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequestMembers3 {
    pub member_id: String,
    pub group_instance_id: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequest4 {
    pub group_id: String,
    pub members: Optional<Vec<LeaveGroupRequestMembers4>>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequestMembers4 {
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse0 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub members: Optional<Vec<LeaveGroupResponseMembers3>>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponseMembers3 {
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub members: Optional<Vec<LeaveGroupResponseMembers4>>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponseMembers4 {
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub error_code: Int16,
    pub tag_buffer: Optional<TagBuffer>,
}

impl TryFrom<LeaveGroupRequest4> for LeaveGroupRequest0 {
    type Error = Error;
    fn try_from(latest: LeaveGroupRequest4) -> Result<Self, Self::Error> {
        if latest.members.is_some() {
            return Err(Error::OldKafkaVersion("LeaveGroupRequest", 0, "members"));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("LeaveGroupRequest", 0, "tag_buffer"));
        }
        Ok(LeaveGroupRequest0 {
            group_id: latest.group_id,
            ..LeaveGroupRequest0::default()
        })
    }
}

impl TryFrom<LeaveGroupRequest4> for LeaveGroupRequest1 {
    type Error = Error;
    fn try_from(latest: LeaveGroupRequest4) -> Result<Self, Self::Error> {
        if latest.members.is_some() {
            return Err(Error::OldKafkaVersion("LeaveGroupRequest", 1, "members"));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("LeaveGroupRequest", 1, "tag_buffer"));
        }
        Ok(LeaveGroupRequest1 {
            group_id: latest.group_id,
            ..LeaveGroupRequest1::default()
        })
    }
}

impl TryFrom<LeaveGroupRequest4> for LeaveGroupRequest2 {
    type Error = Error;
    fn try_from(latest: LeaveGroupRequest4) -> Result<Self, Self::Error> {
        if latest.members.is_some() {
            return Err(Error::OldKafkaVersion("LeaveGroupRequest", 2, "members"));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("LeaveGroupRequest", 2, "tag_buffer"));
        }
        Ok(LeaveGroupRequest2 {
            group_id: latest.group_id,
            ..LeaveGroupRequest2::default()
        })
    }
}

impl TryFrom<LeaveGroupRequest4> for LeaveGroupRequest3 {
    type Error = Error;
    fn try_from(latest: LeaveGroupRequest4) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("LeaveGroupRequest", 3, "tag_buffer"));
        }
        Ok(LeaveGroupRequest3 {
            group_id: latest.group_id,
            members: latest
                .members
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
        })
    }
}

impl TryFrom<LeaveGroupRequestMembers4> for LeaveGroupRequestMembers3 {
    type Error = Error;
    fn try_from(latest: LeaveGroupRequestMembers4) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "LeaveGroupRequestMembers",
                3,
                "tag_buffer",
            ));
        }
        Ok(LeaveGroupRequestMembers3 {
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
        })
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
