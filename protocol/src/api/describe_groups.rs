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
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
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
                DescribeGroupsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DescribeGroupsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &DescribeGroupsRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &DescribeGroupsRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &DescribeGroupsRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &DescribeGroupsRequest3::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &DescribeGroupsRequest4::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DescribeGroupsResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response =
            match version {
                0 => DescribeGroupsResponse0::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                1 => DescribeGroupsResponse1::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                2 => DescribeGroupsResponse2::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                3 => DescribeGroupsResponse3::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                4 => DescribeGroupsResponse4::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                5 => DescribeGroupsResponse::deserialize(buf, Self::is_flexible_version(version)),
                _ => DescribeGroupsResponse::deserialize(buf, Self::is_flexible_version(version)),
            };
        (header.correlation, response)
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
    pub groups: Vec<String>,
    pub include_authorized_operations: Optional<Boolean>,
    pub tag_buffer: Optional<TagBuffer>,
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
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroups5 {
    pub error_code: Int16,
    pub group_id: String,
    pub group_state: String,
    pub protocol_type: String,
    pub protocol_data: String,
    pub members: Vec<DescribeGroupsResponseGroupsMembers5>,
    pub authorized_operations: Optional<Int32>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroupsMembers5 {
    pub member_id: String,
    pub group_instance_id: Optional<NullableString>,
    pub client_id: String,
    pub client_host: String,
    pub member_metadata: KafkaBytes,
    pub member_assignment: KafkaBytes,
    pub tag_buffer: Optional<TagBuffer>,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeGroupsRequest",
                0,
                "tag_buffer",
            ));
        }
        Ok(DescribeGroupsRequest0 {
            groups: latest.groups,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeGroupsRequest",
                1,
                "tag_buffer",
            ));
        }
        Ok(DescribeGroupsRequest1 {
            groups: latest.groups,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeGroupsRequest",
                2,
                "tag_buffer",
            ));
        }
        Ok(DescribeGroupsRequest2 {
            groups: latest.groups,
        })
    }
}

impl TryFrom<DescribeGroupsRequest5> for DescribeGroupsRequest3 {
    type Error = Error;
    fn try_from(latest: DescribeGroupsRequest5) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeGroupsRequest",
                3,
                "tag_buffer",
            ));
        }
        Ok(DescribeGroupsRequest3 {
            groups: latest.groups,
            include_authorized_operations: latest.include_authorized_operations,
        })
    }
}

impl TryFrom<DescribeGroupsRequest5> for DescribeGroupsRequest4 {
    type Error = Error;
    fn try_from(latest: DescribeGroupsRequest5) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeGroupsRequest",
                4,
                "tag_buffer",
            ));
        }
        Ok(DescribeGroupsRequest4 {
            groups: latest.groups,
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
            group_id: older.group_id,
            group_state: older.group_state,
            protocol_type: older.protocol_type,
            protocol_data: older.protocol_data,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponseGroups5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers0> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers0) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id,
            client_id: older.client_id,
            client_host: older.client_host,
            member_metadata: older.member_metadata,
            member_assignment: older.member_assignment,
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}

impl From<DescribeGroupsResponse1> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse1) -> Self {
        DescribeGroupsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponse5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroups1> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups1) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id,
            group_state: older.group_state,
            protocol_type: older.protocol_type,
            protocol_data: older.protocol_data,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponseGroups5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers1> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers1) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id,
            client_id: older.client_id,
            client_host: older.client_host,
            member_metadata: older.member_metadata,
            member_assignment: older.member_assignment,
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}

impl From<DescribeGroupsResponse2> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse2) -> Self {
        DescribeGroupsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponse5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroups2> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups2) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id,
            group_state: older.group_state,
            protocol_type: older.protocol_type,
            protocol_data: older.protocol_data,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponseGroups5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers2> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers2) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id,
            client_id: older.client_id,
            client_host: older.client_host,
            member_metadata: older.member_metadata,
            member_assignment: older.member_assignment,
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}

impl From<DescribeGroupsResponse3> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse3) -> Self {
        DescribeGroupsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponse5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroups3> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups3) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id,
            group_state: older.group_state,
            protocol_type: older.protocol_type,
            protocol_data: older.protocol_data,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            authorized_operations: older.authorized_operations,
            ..DescribeGroupsResponseGroups5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers3> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers3) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id,
            client_id: older.client_id,
            client_host: older.client_host,
            member_metadata: older.member_metadata,
            member_assignment: older.member_assignment,
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}

impl From<DescribeGroupsResponse4> for DescribeGroupsResponse5 {
    fn from(older: DescribeGroupsResponse4) -> Self {
        DescribeGroupsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            ..DescribeGroupsResponse5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroups4> for DescribeGroupsResponseGroups5 {
    fn from(older: DescribeGroupsResponseGroups4) -> Self {
        DescribeGroupsResponseGroups5 {
            error_code: older.error_code,
            group_id: older.group_id,
            group_state: older.group_state,
            protocol_type: older.protocol_type,
            protocol_data: older.protocol_data,
            members: older.members.into_iter().map(|el| el.into()).collect(),
            authorized_operations: older.authorized_operations,
            ..DescribeGroupsResponseGroups5::default()
        }
    }
}

impl From<DescribeGroupsResponseGroupsMembers4> for DescribeGroupsResponseGroupsMembers5 {
    fn from(older: DescribeGroupsResponseGroupsMembers4) -> Self {
        DescribeGroupsResponseGroupsMembers5 {
            member_id: older.member_id,
            group_instance_id: older.group_instance_id,
            client_id: older.client_id,
            client_host: older.client_host,
            member_metadata: older.member_metadata,
            member_assignment: older.member_assignment,
            ..DescribeGroupsResponseGroupsMembers5::default()
        }
    }
}
