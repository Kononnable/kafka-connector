use super::prelude::*;

pub type DescribeGroupsRequest = DescribeGroupsRequest5;
pub type DescribeGroupsResponse = DescribeGroupsResponse5;
impl ApiCall for DescribeGroupsRequest {
    type Response = DescribeGroupsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        5
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeGroups
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&DescribeGroupsRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&DescribeGroupsRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&DescribeGroupsRequest2::try_from(self)?, buf),
            3 => ToBytes::serialize(&DescribeGroupsRequest3::try_from(self)?, buf),
            4 => ToBytes::serialize(&DescribeGroupsRequest4::try_from(self)?, buf),
            5 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> DescribeGroupsResponse {
        match version {
            0 => DescribeGroupsResponse0::deserialize(buf).into(),
            1 => DescribeGroupsResponse1::deserialize(buf).into(),
            2 => DescribeGroupsResponse2::deserialize(buf).into(),
            3 => DescribeGroupsResponse3::deserialize(buf).into(),
            4 => DescribeGroupsResponse4::deserialize(buf).into(),
            5 => DescribeGroupsResponse::deserialize(buf),
            _ => DescribeGroupsResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeGroupsRequest0 {
    pub groups: Vec<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeGroupsRequest1 {
    pub groups: Vec<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeGroupsRequest2 {
    pub groups: Vec<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeGroupsRequest3 {
    pub groups: Vec<String>,
    pub include_authorized_operations: Optional<Boolean>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeGroupsRequest4 {
    pub groups: Vec<String>,
    pub include_authorized_operations: Optional<Boolean>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeGroupsRequest5 {
    pub groups: Vec<CompactString>,
    pub include_authorized_operations: Optional<Boolean>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponse0 {
    pub groups: Vec<DescribeGroupsResponseGroups0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroups0 {
    pub error_code: Int16,
    pub group_id: String,
    pub group_state: String,
    pub protocol_type: String,
    pub protocol_data: String,
    pub members: Vec<DescribeGroupsResponseGroupsMembers0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroupsMembers0 {
    pub member_id: String,
    pub client_id: String,
    pub client_host: String,
    pub member_metadata: KafkaBytes,
    pub member_assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub groups: Vec<DescribeGroupsResponseGroups1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroups1 {
    pub error_code: Int16,
    pub group_id: String,
    pub group_state: String,
    pub protocol_type: String,
    pub protocol_data: String,
    pub members: Vec<DescribeGroupsResponseGroupsMembers1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroupsMembers1 {
    pub member_id: String,
    pub client_id: String,
    pub client_host: String,
    pub member_metadata: KafkaBytes,
    pub member_assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub groups: Vec<DescribeGroupsResponseGroups2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroups2 {
    pub error_code: Int16,
    pub group_id: String,
    pub group_state: String,
    pub protocol_type: String,
    pub protocol_data: String,
    pub members: Vec<DescribeGroupsResponseGroupsMembers2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroupsMembers2 {
    pub member_id: String,
    pub client_id: String,
    pub client_host: String,
    pub member_metadata: KafkaBytes,
    pub member_assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub groups: Vec<DescribeGroupsResponseGroups3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroups3 {
    pub error_code: Int16,
    pub group_id: String,
    pub group_state: String,
    pub protocol_type: String,
    pub protocol_data: String,
    pub members: Vec<DescribeGroupsResponseGroupsMembers3>,
    pub authorized_operations: Optional<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroupsMembers3 {
    pub member_id: String,
    pub client_id: String,
    pub client_host: String,
    pub member_metadata: KafkaBytes,
    pub member_assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub groups: Vec<DescribeGroupsResponseGroups4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroups4 {
    pub error_code: Int16,
    pub group_id: String,
    pub group_state: String,
    pub protocol_type: String,
    pub protocol_data: String,
    pub members: Vec<DescribeGroupsResponseGroupsMembers4>,
    pub authorized_operations: Optional<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroupsMembers4 {
    pub member_id: String,
    pub group_instance_id: Optional<NullableString>,
    pub client_id: String,
    pub client_host: String,
    pub member_metadata: KafkaBytes,
    pub member_assignment: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub groups: Vec<DescribeGroupsResponseGroups5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroups5 {
    pub error_code: Int16,
    pub group_id: CompactString,
    pub group_state: CompactString,
    pub protocol_type: CompactString,
    pub protocol_data: CompactString,
    pub members: Vec<DescribeGroupsResponseGroupsMembers5>,
    pub authorized_operations: Optional<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroupsMembers5 {
    pub member_id: CompactString,
    pub group_instance_id: Optional<CompactNullableString>,
    pub client_id: CompactString,
    pub client_host: CompactString,
    pub member_metadata: CompactBytes,
    pub member_assignment: CompactBytes,
}

impl TryFrom<DescribeGroupsRequest5> for DescribeGroupsRequest0 {
    type Error = Error;
    fn try_from(latest: DescribeGroupsRequest5) -> Result<Self, Self::Error> {
        if latest.include_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeGroupsRequest",
                0,
                "include_authorized_operations",
            ));
        }
        Ok(DescribeGroupsRequest0 {
            groups: latest.groups.into_iter().map(|ele| ele.into()).collect(),
        })
    }
}

impl TryFrom<DescribeGroupsRequest5> for DescribeGroupsRequest1 {
    type Error = Error;
    fn try_from(latest: DescribeGroupsRequest5) -> Result<Self, Self::Error> {
        if latest.include_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeGroupsRequest",
                1,
                "include_authorized_operations",
            ));
        }
        Ok(DescribeGroupsRequest1 {
            groups: latest.groups.into_iter().map(|ele| ele.into()).collect(),
        })
    }
}

impl TryFrom<DescribeGroupsRequest5> for DescribeGroupsRequest2 {
    type Error = Error;
    fn try_from(latest: DescribeGroupsRequest5) -> Result<Self, Self::Error> {
        if latest.include_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeGroupsRequest",
                2,
                "include_authorized_operations",
            ));
        }
        Ok(DescribeGroupsRequest2 {
            groups: latest.groups.into_iter().map(|ele| ele.into()).collect(),
        })
    }
}

impl TryFrom<DescribeGroupsRequest5> for DescribeGroupsRequest3 {
    type Error = Error;
    fn try_from(latest: DescribeGroupsRequest5) -> Result<Self, Self::Error> {
        Ok(DescribeGroupsRequest3 {
            groups: latest.groups.into_iter().map(|ele| ele.into()).collect(),
            include_authorized_operations: latest.include_authorized_operations,
        })
    }
}

impl TryFrom<DescribeGroupsRequest5> for DescribeGroupsRequest4 {
    type Error = Error;
    fn try_from(latest: DescribeGroupsRequest5) -> Result<Self, Self::Error> {
        Ok(DescribeGroupsRequest4 {
            groups: latest.groups.into_iter().map(|ele| ele.into()).collect(),
            include_authorized_operations: latest.include_authorized_operations,
        })
    }
}

impl From<DescribeGroupsResponse0> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse0) -> Self {
        DescribeGroupsResponse5 {
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponse5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroups0> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups0) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id.into(),
            group_state: older.group_state.into(),
            protocol_type: older.protocol_type.into(),
            protocol_data: older.protocol_data.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponseGroups5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers0> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers0) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id.into(),
            client_id: older.client_id.into(),
            client_host: older.client_host.into(),
            member_metadata: older.member_metadata.into(),
            member_assignment: older.member_assignment.into(),
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}

impl From<DescribeGroupsResponse1> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse1) -> Self {
        DescribeGroupsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeGroupsResponseGroups1> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups1) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id.into(),
            group_state: older.group_state.into(),
            protocol_type: older.protocol_type.into(),
            protocol_data: older.protocol_data.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponseGroups5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers1> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers1) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id.into(),
            client_id: older.client_id.into(),
            client_host: older.client_host.into(),
            member_metadata: older.member_metadata.into(),
            member_assignment: older.member_assignment.into(),
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}

impl From<DescribeGroupsResponse2> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse2) -> Self {
        DescribeGroupsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeGroupsResponseGroups2> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups2) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id.into(),
            group_state: older.group_state.into(),
            protocol_type: older.protocol_type.into(),
            protocol_data: older.protocol_data.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponseGroups5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers2> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers2) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id.into(),
            client_id: older.client_id.into(),
            client_host: older.client_host.into(),
            member_metadata: older.member_metadata.into(),
            member_assignment: older.member_assignment.into(),
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}

impl From<DescribeGroupsResponse3> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse3) -> Self {
        DescribeGroupsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeGroupsResponseGroups3> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups3) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id.into(),
            group_state: older.group_state.into(),
            protocol_type: older.protocol_type.into(),
            protocol_data: older.protocol_data.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            authorized_operations: older.authorized_operations,
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers3> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers3) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id.into(),
            client_id: older.client_id.into(),
            client_host: older.client_host.into(),
            member_metadata: older.member_metadata.into(),
            member_assignment: older.member_assignment.into(),
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}

impl From<DescribeGroupsResponse4> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse4) -> Self {
        DescribeGroupsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeGroupsResponseGroups4> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups4) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id.into(),
            group_state: older.group_state.into(),
            protocol_type: older.protocol_type.into(),
            protocol_data: older.protocol_data.into(),
            members: older.members.into_iter().map(|el| el.into()).collect(),
            authorized_operations: older.authorized_operations,
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers4> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers4) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id.into(),
            group_instance_id: older.group_instance_id.map(|val| val.into()),
            client_id: older.client_id.into(),
            client_host: older.client_host.into(),
            member_metadata: older.member_metadata.into(),
            member_assignment: older.member_assignment.into(),
        }
    }
}
