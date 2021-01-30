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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&JoinGroupRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&JoinGroupRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&JoinGroupRequest2::try_from(self)?, buf),
            3 => ToBytes::serialize(&JoinGroupRequest3::try_from(self)?, buf),
            4 => ToBytes::serialize(&JoinGroupRequest4::try_from(self)?, buf),
            5 => ToBytes::serialize(&JoinGroupRequest5::try_from(self)?, buf),
            6 => ToBytes::serialize(&JoinGroupRequest6::try_from(self)?, buf),
            7 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> JoinGroupResponse {
        match version {
            0 => JoinGroupResponse0::deserialize(buf).into(),
            1 => JoinGroupResponse1::deserialize(buf).into(),
            2 => JoinGroupResponse2::deserialize(buf).into(),
            3 => JoinGroupResponse3::deserialize(buf).into(),
            4 => JoinGroupResponse4::deserialize(buf).into(),
            5 => JoinGroupResponse5::deserialize(buf).into(),
            6 => JoinGroupResponse6::deserialize(buf).into(),
            7 => JoinGroupResponse::deserialize(buf),
            _ => JoinGroupResponse::deserialize(buf),
        }
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
    pub rebalance_timeout_ms: Optional<Int32>,
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
    pub rebalance_timeout_ms: Optional<Int32>,
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
    pub rebalance_timeout_ms: Optional<Int32>,
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
    pub rebalance_timeout_ms: Optional<Int32>,
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
    pub rebalance_timeout_ms: Optional<Int32>,
    pub member_id: String,
    pub group_instance_id: Optional<NullableString>,
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
    pub group_id: CompactString,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Optional<Int32>,
    pub member_id: CompactString,
    pub group_instance_id: Optional<CompactNullableString>,
    pub protocol_type: CompactString,
    pub protocols: Vec<JoinGroupRequestProtocols6>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols6 {
    pub name: CompactString,
    pub metadata: CompactBytes,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequest7 {
    pub group_id: CompactString,
    pub session_timeout_ms: Int32,
    pub rebalance_timeout_ms: Optional<Int32>,
    pub member_id: CompactString,
    pub group_instance_id: Optional<CompactNullableString>,
    pub protocol_type: CompactString,
    pub protocols: Vec<JoinGroupRequestProtocols7>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct JoinGroupRequestProtocols7 {
    pub name: CompactString,
    pub metadata: CompactBytes,
    pub tag_buffer: Optional<TagBuffer>,
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
    pub throttle_time_ms: Optional<Int32>,
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
    pub throttle_time_ms: Optional<Int32>,
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
    pub throttle_time_ms: Optional<Int32>,
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
    pub throttle_time_ms: Optional<Int32>,
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
    pub group_instance_id: Optional<NullableString>,
    pub metadata: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse6 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_name: CompactString,
    pub leader: CompactString,
    pub member_id: CompactString,
    pub members: Vec<JoinGroupResponseMembers6>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers6 {
    pub member_id: CompactString,
    pub group_instance_id: Optional<CompactNullableString>,
    pub metadata: CompactBytes,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse7 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub generation_id: Int32,
    pub protocol_type: Optional<CompactNullableString>,
    pub protocol_name: CompactNullableString,
    pub leader: CompactString,
    pub member_id: CompactString,
    pub members: Vec<JoinGroupResponseMembers7>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers7 {
    pub member_id: CompactString,
    pub group_instance_id: Optional<CompactNullableString>,
    pub metadata: CompactBytes,
    pub tag_buffer: Optional<TagBuffer>,
}

impl TryFrom<JoinGroupRequest7> for JoinGroupRequest0 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequest7) -> Result<Self, Self::Error> {
        if latest.rebalance_timeout_ms.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequest",
                0,
                "rebalance_timeout_ms",
            ));
        }
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequest",
                0,
                "group_instance_id",
            ));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("JoinGroupRequest", 0, "tag_buffer"));
        }
        Ok(JoinGroupRequest0 {
            group_id: latest.group_id.into(),
            session_timeout_ms: latest.session_timeout_ms,
            member_id: latest.member_id.into(),
            protocol_type: latest.protocol_type.into(),
            protocols: latest
                .protocols
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols0 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequestProtocols7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequestProtocols",
                0,
                "tag_buffer",
            ));
        }
        Ok(JoinGroupRequestProtocols0 {
            name: latest.name.into(),
            metadata: latest.metadata.into(),
        })
    }
}

impl TryFrom<JoinGroupRequest7> for JoinGroupRequest1 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequest7) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequest",
                1,
                "group_instance_id",
            ));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("JoinGroupRequest", 1, "tag_buffer"));
        }
        Ok(JoinGroupRequest1 {
            group_id: latest.group_id.into(),
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id.into(),
            protocol_type: latest.protocol_type.into(),
            protocols: latest
                .protocols
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols1 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequestProtocols7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequestProtocols",
                1,
                "tag_buffer",
            ));
        }
        Ok(JoinGroupRequestProtocols1 {
            name: latest.name.into(),
            metadata: latest.metadata.into(),
        })
    }
}

impl TryFrom<JoinGroupRequest7> for JoinGroupRequest2 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequest7) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequest",
                2,
                "group_instance_id",
            ));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("JoinGroupRequest", 2, "tag_buffer"));
        }
        Ok(JoinGroupRequest2 {
            group_id: latest.group_id.into(),
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id.into(),
            protocol_type: latest.protocol_type.into(),
            protocols: latest
                .protocols
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols2 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequestProtocols7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequestProtocols",
                2,
                "tag_buffer",
            ));
        }
        Ok(JoinGroupRequestProtocols2 {
            name: latest.name.into(),
            metadata: latest.metadata.into(),
        })
    }
}

impl TryFrom<JoinGroupRequest7> for JoinGroupRequest3 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequest7) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequest",
                3,
                "group_instance_id",
            ));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("JoinGroupRequest", 3, "tag_buffer"));
        }
        Ok(JoinGroupRequest3 {
            group_id: latest.group_id.into(),
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id.into(),
            protocol_type: latest.protocol_type.into(),
            protocols: latest
                .protocols
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols3 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequestProtocols7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequestProtocols",
                3,
                "tag_buffer",
            ));
        }
        Ok(JoinGroupRequestProtocols3 {
            name: latest.name.into(),
            metadata: latest.metadata.into(),
        })
    }
}

impl TryFrom<JoinGroupRequest7> for JoinGroupRequest4 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequest7) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequest",
                4,
                "group_instance_id",
            ));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("JoinGroupRequest", 4, "tag_buffer"));
        }
        Ok(JoinGroupRequest4 {
            group_id: latest.group_id.into(),
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id.into(),
            protocol_type: latest.protocol_type.into(),
            protocols: latest
                .protocols
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols4 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequestProtocols7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequestProtocols",
                4,
                "tag_buffer",
            ));
        }
        Ok(JoinGroupRequestProtocols4 {
            name: latest.name.into(),
            metadata: latest.metadata.into(),
        })
    }
}

impl TryFrom<JoinGroupRequest7> for JoinGroupRequest5 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequest7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion("JoinGroupRequest", 5, "tag_buffer"));
        }
        Ok(JoinGroupRequest5 {
            group_id: latest.group_id.into(),
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id.into(),
            group_instance_id: latest.group_instance_id.map(|val| val.into()),
            protocol_type: latest.protocol_type.into(),
            protocols: latest
                .protocols
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols5 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequestProtocols7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "JoinGroupRequestProtocols",
                5,
                "tag_buffer",
            ));
        }
        Ok(JoinGroupRequestProtocols5 {
            name: latest.name.into(),
            metadata: latest.metadata.into(),
        })
    }
}

impl TryFrom<JoinGroupRequest7> for JoinGroupRequest6 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequest7) -> Result<Self, Self::Error> {
        Ok(JoinGroupRequest6 {
            group_id: latest.group_id,
            session_timeout_ms: latest.session_timeout_ms,
            rebalance_timeout_ms: latest.rebalance_timeout_ms,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
            protocol_type: latest.protocol_type,
            protocols: latest
                .protocols
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            tag_buffer: latest.tag_buffer,
        })
    }
}

impl TryFrom<JoinGroupRequestProtocols7> for JoinGroupRequestProtocols6 {
    type Error = Error;
    fn try_from(latest: JoinGroupRequestProtocols7) -> Result<Self, Self::Error> {
        Ok(JoinGroupRequestProtocols6 {
            name: latest.name,
            metadata: latest.metadata,
            tag_buffer: latest.tag_buffer,
        })
    }
}

impl From<JoinGroupResponse0> for JoinGroupResponse7 {
    fn from(older: JoinGroupResponse0) -> Self {
        JoinGroupResponse7 {
            error_code: older.error_code,
            generation_id: older.generation_id,
            protocol_name: older.protocol_name.into(),
            leader: older.leader.into(),
            member_id: older.member_id.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers0> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers0) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id.into(),
            metadata: older.metadata.into(),
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
            leader: older.leader.into(),
            member_id: older.member_id.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers1> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers1) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id.into(),
            metadata: older.metadata.into(),
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
            leader: older.leader.into(),
            member_id: older.member_id.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers2> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers2) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id.into(),
            metadata: older.metadata.into(),
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
            leader: older.leader.into(),
            member_id: older.member_id.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers3> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers3) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id.into(),
            metadata: older.metadata.into(),
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
            leader: older.leader.into(),
            member_id: older.member_id.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers4> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers4) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id.into(),
            metadata: older.metadata.into(),
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
            leader: older.leader.into(),
            member_id: older.member_id.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..JoinGroupResponse7::default()
        }
    }
}

impl From<JoinGroupResponseMembers5> for JoinGroupResponseMembers7 {
    fn from(older: JoinGroupResponseMembers5) -> Self {
        JoinGroupResponseMembers7 {
            member_id: older.member_id.into(),
            group_instance_id: older.group_instance_id.map(|val| val.into()),
            metadata: older.metadata.into(),
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
