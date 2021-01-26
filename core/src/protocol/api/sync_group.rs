use super::prelude::*;

pub type SyncGroupRequest = SyncGroupRequest5;
pub type SyncGroupResponse = SyncGroupResponse5;
pub fn serialize_sync_group_request(
    data: SyncGroupRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&SyncGroupRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&SyncGroupRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&SyncGroupRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&SyncGroupRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&SyncGroupRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_sync_group_response(version: i32, buf: &mut Bytes) -> SyncGroupResponse {
    match version {
        0 => SyncGroupResponse0::deserialize(buf).into(),
        1 => SyncGroupResponse1::deserialize(buf).into(),
        2 => SyncGroupResponse2::deserialize(buf).into(),
        3 => SyncGroupResponse3::deserialize(buf).into(),
        4 => SyncGroupResponse4::deserialize(buf).into(),
        5 => SyncGroupResponse::deserialize(buf),
        _ => SyncGroupResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequest0 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub assignments: Vec<SyncGroupRequestAssignments0>,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequestAssignments0 {
    pub member_id: String,
    pub assignment: KafkaBytes,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequest1 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub assignments: Vec<SyncGroupRequestAssignments1>,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequestAssignments1 {
    pub member_id: String,
    pub assignment: KafkaBytes,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequest2 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub assignments: Vec<SyncGroupRequestAssignments2>,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequestAssignments2 {
    pub member_id: String,
    pub assignment: KafkaBytes,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequest3 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: Optional<NullableString>,
    pub assignments: Vec<SyncGroupRequestAssignments3>,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequestAssignments3 {
    pub member_id: String,
    pub assignment: KafkaBytes,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequest4 {
    pub group_id: CompactString,
    pub generation_id: Int32,
    pub member_id: CompactString,
    pub group_instance_id: Optional<CompactNullableString>,
    pub assignments: Vec<SyncGroupRequestAssignments4>,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequestAssignments4 {
    pub member_id: CompactString,
    pub assignment: CompactBytes,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequest5 {
    pub group_id: CompactString,
    pub generation_id: Int32,
    pub member_id: CompactString,
    pub group_instance_id: Optional<CompactNullableString>,
    pub protocol_type: Optional<CompactNullableString>,
    pub protocol_name: Optional<CompactNullableString>,
    pub assignments: Vec<SyncGroupRequestAssignments5>,
}

#[derive(Default, ToBytes)]
pub struct SyncGroupRequestAssignments5 {
    pub member_id: CompactString,
    pub assignment: CompactBytes,
}

#[derive(Default, FromBytes)]
pub struct SyncGroupResponse0 {
    pub error_code: Int16,
    pub assignment: KafkaBytes,
}

#[derive(Default, FromBytes)]
pub struct SyncGroupResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub assignment: KafkaBytes,
}

#[derive(Default, FromBytes)]
pub struct SyncGroupResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub assignment: KafkaBytes,
}

#[derive(Default, FromBytes)]
pub struct SyncGroupResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub assignment: KafkaBytes,
}

#[derive(Default, FromBytes)]
pub struct SyncGroupResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub assignment: CompactBytes,
}

#[derive(Default, FromBytes)]
pub struct SyncGroupResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub protocol_type: Optional<CompactNullableString>,
    pub protocol_name: Optional<CompactNullableString>,
    pub assignment: CompactBytes,
}

impl TryFrom<SyncGroupRequest5> for SyncGroupRequest0 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequest5) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                0,
                "group_instance_id",
            ));
        }
        if latest.protocol_type.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                0,
                "protocol_type",
            ));
        }
        if latest.protocol_name.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                0,
                "protocol_name",
            ));
        }
        Ok(SyncGroupRequest0 {
            group_id: latest.group_id.into(),
            generation_id: latest.generation_id,
            member_id: latest.member_id.into(),
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments0 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequestAssignments5) -> Result<Self, Self::Error> {
        Ok(SyncGroupRequestAssignments0 {
            member_id: latest.member_id.into(),
            assignment: latest.assignment.into(),
        })
    }
}

impl TryFrom<SyncGroupRequest5> for SyncGroupRequest1 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequest5) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                1,
                "group_instance_id",
            ));
        }
        if latest.protocol_type.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                1,
                "protocol_type",
            ));
        }
        if latest.protocol_name.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                1,
                "protocol_name",
            ));
        }
        Ok(SyncGroupRequest1 {
            group_id: latest.group_id.into(),
            generation_id: latest.generation_id,
            member_id: latest.member_id.into(),
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments1 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequestAssignments5) -> Result<Self, Self::Error> {
        Ok(SyncGroupRequestAssignments1 {
            member_id: latest.member_id.into(),
            assignment: latest.assignment.into(),
        })
    }
}

impl TryFrom<SyncGroupRequest5> for SyncGroupRequest2 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequest5) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                2,
                "group_instance_id",
            ));
        }
        if latest.protocol_type.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                2,
                "protocol_type",
            ));
        }
        if latest.protocol_name.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                2,
                "protocol_name",
            ));
        }
        Ok(SyncGroupRequest2 {
            group_id: latest.group_id.into(),
            generation_id: latest.generation_id,
            member_id: latest.member_id.into(),
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments2 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequestAssignments5) -> Result<Self, Self::Error> {
        Ok(SyncGroupRequestAssignments2 {
            member_id: latest.member_id.into(),
            assignment: latest.assignment.into(),
        })
    }
}

impl TryFrom<SyncGroupRequest5> for SyncGroupRequest3 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequest5) -> Result<Self, Self::Error> {
        if latest.protocol_type.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                3,
                "protocol_type",
            ));
        }
        if latest.protocol_name.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                3,
                "protocol_name",
            ));
        }
        Ok(SyncGroupRequest3 {
            group_id: latest.group_id.into(),
            generation_id: latest.generation_id,
            member_id: latest.member_id.into(),
            group_instance_id: latest.group_instance_id.map(|val| val),
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments3 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequestAssignments5) -> Result<Self, Self::Error> {
        Ok(SyncGroupRequestAssignments3 {
            member_id: latest.member_id.into(),
            assignment: latest.assignment.into(),
        })
    }
}

impl TryFrom<SyncGroupRequest5> for SyncGroupRequest4 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequest5) -> Result<Self, Self::Error> {
        if latest.protocol_type.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                4,
                "protocol_type",
            ));
        }
        if latest.protocol_name.is_some() {
            return Err(Error::OldKafkaVersion(
                "SyncGroupRequest",
                4,
                "protocol_name",
            ));
        }
        Ok(SyncGroupRequest4 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments4 {
    type Error = Error;
    fn try_from(latest: SyncGroupRequestAssignments5) -> Result<Self, Self::Error> {
        Ok(SyncGroupRequestAssignments4 {
            member_id: latest.member_id,
            assignment: latest.assignment,
        })
    }
}

impl From<SyncGroupResponse0> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse0) -> Self {
        SyncGroupResponse5 {
            error_code: older.error_code,
            assignment: older.assignment.into(),
            ..SyncGroupResponse5::default()
        }
    }
}

impl From<SyncGroupResponse1> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse1) -> Self {
        SyncGroupResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            assignment: older.assignment.into(),
            ..SyncGroupResponse5::default()
        }
    }
}

impl From<SyncGroupResponse2> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse2) -> Self {
        SyncGroupResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            assignment: older.assignment.into(),
            ..SyncGroupResponse5::default()
        }
    }
}

impl From<SyncGroupResponse3> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse3) -> Self {
        SyncGroupResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            assignment: older.assignment.into(),
            ..SyncGroupResponse5::default()
        }
    }
}

impl From<SyncGroupResponse4> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse4) -> Self {
        SyncGroupResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            assignment: older.assignment,
            ..SyncGroupResponse5::default()
        }
    }
}
