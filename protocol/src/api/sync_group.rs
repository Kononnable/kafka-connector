use super::prelude::*;

pub type SyncGroupRequest = SyncGroupRequest5;
pub type SyncGroupResponse = SyncGroupResponse5;
impl ApiCall for SyncGroupRequest {
    type Response = SyncGroupResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        5
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::SyncGroup
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => true,
            5 => true,
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
                SyncGroupRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                SyncGroupRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &SyncGroupRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &SyncGroupRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &SyncGroupRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &SyncGroupRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &SyncGroupRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, SyncGroupResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => SyncGroupResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => SyncGroupResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => SyncGroupResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => SyncGroupResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => SyncGroupResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => SyncGroupResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => SyncGroupResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequest0 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub assignments: Vec<SyncGroupRequestAssignments0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequestAssignments0 {
    pub member_id: String,
    pub assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequest1 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub assignments: Vec<SyncGroupRequestAssignments1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequestAssignments1 {
    pub member_id: String,
    pub assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequest2 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub assignments: Vec<SyncGroupRequestAssignments2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequestAssignments2 {
    pub member_id: String,
    pub assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequest3 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub assignments: Vec<SyncGroupRequestAssignments3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequestAssignments3 {
    pub member_id: String,
    pub assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequest4 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub assignments: Vec<SyncGroupRequestAssignments4>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequestAssignments4 {
    pub member_id: String,
    pub assignment: KafkaBytes,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequest5 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub protocol_type: NullableString,
    pub protocol_name: NullableString,
    pub assignments: Vec<SyncGroupRequestAssignments5>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SyncGroupRequestAssignments5 {
    pub member_id: String,
    pub assignment: KafkaBytes,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SyncGroupResponse0 {
    pub error_code: Int16,
    pub assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SyncGroupResponse1 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SyncGroupResponse2 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SyncGroupResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SyncGroupResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub assignment: KafkaBytes,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SyncGroupResponse5 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub protocol_type: Option<NullableString>,
    pub protocol_name: Option<NullableString>,
    pub assignment: KafkaBytes,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<SyncGroupRequest5> for SyncGroupRequest0 {
    fn from(latest: SyncGroupRequest5) -> SyncGroupRequest0 {
        SyncGroupRequest0 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments0 {
    fn from(latest: SyncGroupRequestAssignments5) -> SyncGroupRequestAssignments0 {
        SyncGroupRequestAssignments0 {
            member_id: latest.member_id,
            assignment: latest.assignment,
        }
    }
}

impl From<SyncGroupRequest5> for SyncGroupRequest1 {
    fn from(latest: SyncGroupRequest5) -> SyncGroupRequest1 {
        SyncGroupRequest1 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments1 {
    fn from(latest: SyncGroupRequestAssignments5) -> SyncGroupRequestAssignments1 {
        SyncGroupRequestAssignments1 {
            member_id: latest.member_id,
            assignment: latest.assignment,
        }
    }
}

impl From<SyncGroupRequest5> for SyncGroupRequest2 {
    fn from(latest: SyncGroupRequest5) -> SyncGroupRequest2 {
        SyncGroupRequest2 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments2 {
    fn from(latest: SyncGroupRequestAssignments5) -> SyncGroupRequestAssignments2 {
        SyncGroupRequestAssignments2 {
            member_id: latest.member_id,
            assignment: latest.assignment,
        }
    }
}

impl From<SyncGroupRequest5> for SyncGroupRequest3 {
    fn from(latest: SyncGroupRequest5) -> SyncGroupRequest3 {
        SyncGroupRequest3 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments3 {
    fn from(latest: SyncGroupRequestAssignments5) -> SyncGroupRequestAssignments3 {
        SyncGroupRequestAssignments3 {
            member_id: latest.member_id,
            assignment: latest.assignment,
        }
    }
}

impl From<SyncGroupRequest5> for SyncGroupRequest4 {
    fn from(latest: SyncGroupRequest5) -> SyncGroupRequest4 {
        SyncGroupRequest4 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<SyncGroupRequestAssignments5> for SyncGroupRequestAssignments4 {
    fn from(latest: SyncGroupRequestAssignments5) -> SyncGroupRequestAssignments4 {
        SyncGroupRequestAssignments4 {
            member_id: latest.member_id,
            assignment: latest.assignment,
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<SyncGroupResponse0> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse0) -> Self {
        SyncGroupResponse5 {
            error_code: older.error_code,
            assignment: older.assignment,
            ..SyncGroupResponse5::default()
        }
    }
}

impl From<SyncGroupResponse1> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse1) -> Self {
        SyncGroupResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            assignment: older.assignment,
            ..SyncGroupResponse5::default()
        }
    }
}

impl From<SyncGroupResponse2> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse2) -> Self {
        SyncGroupResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            assignment: older.assignment,
            ..SyncGroupResponse5::default()
        }
    }
}

impl From<SyncGroupResponse3> for SyncGroupResponse5 {
    fn from(older: SyncGroupResponse3) -> Self {
        SyncGroupResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            assignment: older.assignment,
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
            tag_buffer: older.tag_buffer,
            ..SyncGroupResponse5::default()
        }
    }
}
