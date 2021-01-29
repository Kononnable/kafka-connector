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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&LeaveGroupRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&LeaveGroupRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&LeaveGroupRequest2::try_from(self)?, buf),
            3 => ToBytes::serialize(&LeaveGroupRequest3::try_from(self)?, buf),
            4 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> LeaveGroupResponse {
        match version {
            0 => LeaveGroupResponse0::deserialize(buf).into(),
            1 => LeaveGroupResponse1::deserialize(buf).into(),
            2 => LeaveGroupResponse2::deserialize(buf).into(),
            3 => LeaveGroupResponse3::deserialize(buf).into(),
            4 => LeaveGroupResponse::deserialize(buf),
            _ => LeaveGroupResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, ToBytes)]
pub struct LeaveGroupRequest0 {
    pub group_id: String,
    pub member_id: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaveGroupRequest1 {
    pub group_id: String,
    pub member_id: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaveGroupRequest2 {
    pub group_id: String,
    pub member_id: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaveGroupRequest3 {
    pub group_id: String,
    pub members: Optional<Vec<LeaveGroupRequestMembers3>>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaveGroupRequestMembers3 {
    pub member_id: String,
    pub group_instance_id: NullableString,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaveGroupRequest4 {
    pub group_id: CompactString,
    pub members: Optional<Vec<LeaveGroupRequestMembers4>>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaveGroupRequestMembers4 {
    pub member_id: CompactString,
    pub group_instance_id: CompactNullableString,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaveGroupResponse0 {
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaveGroupResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaveGroupResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaveGroupResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub members: Optional<Vec<LeaveGroupResponseMembers3>>,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaveGroupResponseMembers3 {
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaveGroupResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub members: Optional<Vec<LeaveGroupResponseMembers4>>,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaveGroupResponseMembers4 {
    pub member_id: CompactString,
    pub group_instance_id: CompactNullableString,
    pub error_code: Int16,
}

impl TryFrom<LeaveGroupRequest4> for LeaveGroupRequest0 {
    type Error = Error;
    fn try_from(latest: LeaveGroupRequest4) -> Result<Self, Self::Error> {
        if latest.members.is_some() {
            return Err(Error::OldKafkaVersion("LeaveGroupRequest", 0, "members"));
        }
        Ok(LeaveGroupRequest0 {
            group_id: latest.group_id.into(),
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
        Ok(LeaveGroupRequest1 {
            group_id: latest.group_id.into(),
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
        Ok(LeaveGroupRequest2 {
            group_id: latest.group_id.into(),
            ..LeaveGroupRequest2::default()
        })
    }
}

impl TryFrom<LeaveGroupRequest4> for LeaveGroupRequest3 {
    type Error = Error;
    fn try_from(latest: LeaveGroupRequest4) -> Result<Self, Self::Error> {
        Ok(LeaveGroupRequest3 {
            group_id: latest.group_id.into(),
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
        Ok(LeaveGroupRequestMembers3 {
            member_id: latest.member_id.into(),
            group_instance_id: latest.group_instance_id.into(),
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
        }
    }
}

impl From<LeaveGroupResponseMembers3> for LeaveGroupResponseMembers4 {
    fn from(older: LeaveGroupResponseMembers3) -> Self {
        LeaveGroupResponseMembers4 {
            member_id: older.member_id.into(),
            group_instance_id: older.group_instance_id.into(),
            error_code: older.error_code,
        }
    }
}
