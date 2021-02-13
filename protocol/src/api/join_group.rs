use super::prelude::*;

pub type JoinGroupRequest = JoinGroupRequest7;
pub type JoinGroupResponse = JoinGroupResponse7;
impl ApiCall for JoinGroupRequest {
    type Response = JoinGroupResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        7
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::JoinGroup
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => false,
            6 => true,
            7 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                JoinGroupRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                JoinGroupRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &JoinGroupRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &JoinGroupRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &JoinGroupRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &JoinGroupRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &JoinGroupRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &JoinGroupRequest5::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(
                &JoinGroupRequest6::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            7 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, JoinGroupResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => JoinGroupResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => JoinGroupResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => JoinGroupResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => JoinGroupResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => JoinGroupResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => JoinGroupResponse5::deserialize(buf, Self::is_flexible_version(version)).into(),
            6 => JoinGroupResponse6::deserialize(buf, Self::is_flexible_version(version)).into(),
            7 => JoinGroupResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => JoinGroupResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest0 {
    pub group_id: String,
    pub session_timeout_ms: Int32,
    pub member_id: String,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocols0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols0 {
    pub name: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest1 {
    pub group_id: String,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Int32,
    pub member_id: String,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocols1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols1 {
    pub name: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest2 {
    pub group_id: String,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Int32,
    pub member_id: String,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocols2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols2 {
    pub name: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest3 {
    pub group_id: String,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Int32,
    pub member_id: String,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocols3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols3 {
    pub name: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest4 {
    pub group_id: String,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Int32,
    pub member_id: String,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocols4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols4 {
    pub name: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest5 {
    pub group_id: String,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocols5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols5 {
    pub name: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest6 {
    pub group_id: String,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocols6>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols6 {
    pub name: String,
    pub metadata: KafkaBytes,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest7 {
    pub group_id: String,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub protocol_type: String,
    pub protocols: Vec<JoinGroupRequestProtocols7>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols7 {
    pub name: String,
    pub metadata: KafkaBytes,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse0 {
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_name: String,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMembers0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers0 {
    pub member_id: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse1 {
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_name: String,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMembers1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers1 {
    pub member_id: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse2 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_name: String,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMembers2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers2 {
    pub member_id: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_name: String,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMembers3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers3 {
    pub member_id: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_name: String,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMembers4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers4 {
    pub member_id: String,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse5 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_name: String,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMembers5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers5 {
    pub member_id: String,
    pub group_instance_id: Option<NullableString>,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse6 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_name: String,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMembers6>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers6 {
    pub member_id: String,
    pub group_instance_id: Option<NullableString>,
    pub metadata: KafkaBytes,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse7 {
    pub throttle_time_ms: Option<Int32>,
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_type: Option<NullableString>,
    pub protocol_name: NullableString,
    pub leader: String,
    pub member_id: String,
    pub members: Vec<JoinGroupResponseMembers7>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers7 {
    pub member_id: String,
    pub group_instance_id: Option<NullableString>,
    pub metadata: KafkaBytes,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<JoinGroupRequest7> for JoinGroupRequest0 {
    fn from(latest: JoinGroupRequest7) -> JoinGroupRequest0 {
        log::debug!(
            "Using old api format - JoinGroupRequest0, ignoring field rebalance_timeout_ms"
        );
        JoinGroupRequest0 {
            group_id: latest.group_id,
            session_timeout_ms: latest.session_timeout_ms,
            member_id: latest.member_id,
            protocol_type: latest.protocol_type,
            protocols: latest.protocols.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols0 {
    fn from(latest: JoinGroupRequestProtocols7) -> JoinGroupRequestProtocols0 {
        JoinGroupRequestProtocols0 {
            name: latest.name,
            metadata: latest.metadata,
        }
    }
}

impl From<JoinGroupRequest7> for JoinGroupRequest1 {
    fn from(latest: JoinGroupRequest7) -> JoinGroupRequest1 {
        JoinGroupRequest1 {
            group_id: latest.group_id,
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id,
            protocol_type: latest.protocol_type,
            protocols: latest.protocols.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols1 {
    fn from(latest: JoinGroupRequestProtocols7) -> JoinGroupRequestProtocols1 {
        JoinGroupRequestProtocols1 {
            name: latest.name,
            metadata: latest.metadata,
        }
    }
}

impl From<JoinGroupRequest7> for JoinGroupRequest2 {
    fn from(latest: JoinGroupRequest7) -> JoinGroupRequest2 {
        JoinGroupRequest2 {
            group_id: latest.group_id,
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id,
            protocol_type: latest.protocol_type,
            protocols: latest.protocols.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols2 {
    fn from(latest: JoinGroupRequestProtocols7) -> JoinGroupRequestProtocols2 {
        JoinGroupRequestProtocols2 {
            name: latest.name,
            metadata: latest.metadata,
        }
    }
}

impl From<JoinGroupRequest7> for JoinGroupRequest3 {
    fn from(latest: JoinGroupRequest7) -> JoinGroupRequest3 {
        JoinGroupRequest3 {
            group_id: latest.group_id,
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id,
            protocol_type: latest.protocol_type,
            protocols: latest.protocols.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols3 {
    fn from(latest: JoinGroupRequestProtocols7) -> JoinGroupRequestProtocols3 {
        JoinGroupRequestProtocols3 {
            name: latest.name,
            metadata: latest.metadata,
        }
    }
}

impl From<JoinGroupRequest7> for JoinGroupRequest4 {
    fn from(latest: JoinGroupRequest7) -> JoinGroupRequest4 {
        JoinGroupRequest4 {
            group_id: latest.group_id,
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id,
            protocol_type: latest.protocol_type,
            protocols: latest.protocols.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols4 {
    fn from(latest: JoinGroupRequestProtocols7) -> JoinGroupRequestProtocols4 {
        JoinGroupRequestProtocols4 {
            name: latest.name,
            metadata: latest.metadata,
        }
    }
}

impl From<JoinGroupRequest7> for JoinGroupRequest5 {
    fn from(latest: JoinGroupRequest7) -> JoinGroupRequest5 {
        JoinGroupRequest5 {
            group_id: latest.group_id,
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
            protocol_type: latest.protocol_type,
            protocols: latest.protocols.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols5 {
    fn from(latest: JoinGroupRequestProtocols7) -> JoinGroupRequestProtocols5 {
        JoinGroupRequestProtocols5 {
            name: latest.name,
            metadata: latest.metadata,
        }
    }
}

impl From<JoinGroupRequest7> for JoinGroupRequest6 {
    fn from(latest: JoinGroupRequest7) -> JoinGroupRequest6 {
        JoinGroupRequest6 {
            group_id: latest.group_id,
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
            protocol_type: latest.protocol_type,
            protocols: latest.protocols.into_iter().map(|ele| ele.into()).collect(),
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols6 {
    fn from(latest: JoinGroupRequestProtocols7) -> JoinGroupRequestProtocols6 {
        JoinGroupRequestProtocols6 {
            name: latest.name,
            metadata: latest.metadata,
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<JoinGroupResponse0> for JoinGroupResponse7 {
    fn from(older: JoinGroupResponse0) -> Self {
        JoinGroupResponse7 {
            error_code: older.error_code,
            generation_id: older.generation_id,
            protocol_name: older.protocol_name.into(),
            leader: older.leader,
            member_id: older.member_id,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers0> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers0) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id,
            metadata: older.metadata,
            ..JoinGroupResponseMembers7::default()
        }
    }
}

impl From<JoinGroupResponse1> for JoinGroupResponse7 {
    fn from(older: JoinGroupResponse1) -> Self {
        JoinGroupResponse7 {
            error_code: older.error_code,
            generation_id: older.generation_id,
            protocol_name: older.protocol_name.into(),
            leader: older.leader,
            member_id: older.member_id,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers1> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers1) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id,
            metadata: older.metadata,
            ..JoinGroupResponseMembers7::default()
        }
    }
}

impl From<JoinGroupResponse2> for JoinGroupResponse7 {
    fn from(older: JoinGroupResponse2) -> Self {
        JoinGroupResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            generation_id: older.generation_id,
            protocol_name: older.protocol_name.into(),
            leader: older.leader,
            member_id: older.member_id,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers2> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers2) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id,
            metadata: older.metadata,
            ..JoinGroupResponseMembers7::default()
        }
    }
}

impl From<JoinGroupResponse3> for JoinGroupResponse7 {
    fn from(older: JoinGroupResponse3) -> Self {
        JoinGroupResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            generation_id: older.generation_id,
            protocol_name: older.protocol_name.into(),
            leader: older.leader,
            member_id: older.member_id,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers3> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers3) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id,
            metadata: older.metadata,
            ..JoinGroupResponseMembers7::default()
        }
    }
}

impl From<JoinGroupResponse4> for JoinGroupResponse7 {
    fn from(older: JoinGroupResponse4) -> Self {
        JoinGroupResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            generation_id: older.generation_id,
            protocol_name: older.protocol_name.into(),
            leader: older.leader,
            member_id: older.member_id,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers4> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers4) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id,
            metadata: older.metadata,
            ..JoinGroupResponseMembers7::default()
        }
    }
}

impl From<JoinGroupResponse5> for JoinGroupResponse7 {
    fn from(older: JoinGroupResponse5) -> Self {
        JoinGroupResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            generation_id: older.generation_id,
            protocol_name: older.protocol_name.into(),
            leader: older.leader,
            member_id: older.member_id,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers5> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers5) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id,
            group_instance_id: older.group_instance_id,
            metadata: older.metadata,
            ..JoinGroupResponseMembers7::default()
        }
    }
}

impl From<JoinGroupResponse6> for JoinGroupResponse7 {
    fn from(older: JoinGroupResponse6) -> Self {
        JoinGroupResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            generation_id: older.generation_id,
            protocol_name: older.protocol_name.into(),
            leader: older.leader,
            member_id: older.member_id,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            tag_buffer: older.tag_buffer,
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers6> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers6) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id,
            group_instance_id: older.group_instance_id,
            metadata: older.metadata,
            tag_buffer: older.tag_buffer,
        }
    }
}
