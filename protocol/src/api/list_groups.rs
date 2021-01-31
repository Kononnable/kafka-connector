use super::prelude::*;

pub type ListGroupsRequest = ListGroupsRequest4;
pub type ListGroupsResponse = ListGroupsResponse4;
impl ApiCall for ListGroupsRequest {
    type Response = ListGroupsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        4
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ListGroups
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => true,
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
                ListGroupsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                ListGroupsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &ListGroupsRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &ListGroupsRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &ListGroupsRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &ListGroupsRequest3::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, ListGroupsResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => ListGroupsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => ListGroupsResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => ListGroupsResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => ListGroupsResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => ListGroupsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => ListGroupsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListGroupsRequest0 {}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListGroupsRequest1 {}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListGroupsRequest2 {}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListGroupsRequest3 {
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListGroupsRequest4 {
    pub states_filter: Optional<Vec<String>>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponse0 {
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponseGroups0 {
    pub group_id: String,
    pub protocol_type: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponseGroups1 {
    pub group_id: String,
    pub protocol_type: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponseGroups2 {
    pub group_id: String,
    pub protocol_type: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups3>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponseGroups3 {
    pub group_id: String,
    pub protocol_type: String,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups4>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListGroupsResponseGroups4 {
    pub group_id: String,
    pub protocol_type: String,
    pub group_state: Optional<String>,
    pub tag_buffer: Optional<TagBuffer>,
}

impl TryFrom<ListGroupsRequest4> for ListGroupsRequest0 {
    type Error = Error;
    fn try_from(latest: ListGroupsRequest4) -> Result<Self, Self::Error> {
        Ok(ListGroupsRequest0 {})
    }
}

impl TryFrom<ListGroupsRequest4> for ListGroupsRequest1 {
    type Error = Error;
    fn try_from(latest: ListGroupsRequest4) -> Result<Self, Self::Error> {
        Ok(ListGroupsRequest1 {})
    }
}

impl TryFrom<ListGroupsRequest4> for ListGroupsRequest2 {
    type Error = Error;
    fn try_from(latest: ListGroupsRequest4) -> Result<Self, Self::Error> {
        Ok(ListGroupsRequest2 {})
    }
}

impl TryFrom<ListGroupsRequest4> for ListGroupsRequest3 {
    type Error = Error;
    fn try_from(latest: ListGroupsRequest4) -> Result<Self, Self::Error> {
        Ok(ListGroupsRequest3 {
            tag_buffer: latest.tag_buffer,
        })
    }
}

impl From<ListGroupsResponse0> for ListGroupsResponse4 {
    fn from(older: ListGroupsResponse0) -> Self {
        ListGroupsResponse4 {
            error_code: older.error_code,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            ..ListGroupsResponse4::default()
        }
    }
}

impl From<ListGroupsResponseGroups0> for ListGroupsResponseGroups4 {
    fn from(older: ListGroupsResponseGroups0) -> Self {
        ListGroupsResponseGroups4 {
            group_id: older.group_id,
            protocol_type: older.protocol_type,
            ..ListGroupsResponseGroups4::default()
        }
    }
}

impl From<ListGroupsResponse1> for ListGroupsResponse4 {
    fn from(older: ListGroupsResponse1) -> Self {
        ListGroupsResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            ..ListGroupsResponse4::default()
        }
    }
}

impl From<ListGroupsResponseGroups1> for ListGroupsResponseGroups4 {
    fn from(older: ListGroupsResponseGroups1) -> Self {
        ListGroupsResponseGroups4 {
            group_id: older.group_id,
            protocol_type: older.protocol_type,
            ..ListGroupsResponseGroups4::default()
        }
    }
}

impl From<ListGroupsResponse2> for ListGroupsResponse4 {
    fn from(older: ListGroupsResponse2) -> Self {
        ListGroupsResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            ..ListGroupsResponse4::default()
        }
    }
}

impl From<ListGroupsResponseGroups2> for ListGroupsResponseGroups4 {
    fn from(older: ListGroupsResponseGroups2) -> Self {
        ListGroupsResponseGroups4 {
            group_id: older.group_id,
            protocol_type: older.protocol_type,
            ..ListGroupsResponseGroups4::default()
        }
    }
}

impl From<ListGroupsResponse3> for ListGroupsResponse4 {
    fn from(older: ListGroupsResponse3) -> Self {
        ListGroupsResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            groups: older.groups.into_iter().map(|el| el.into()).collect(),
            tag_buffer: older.tag_buffer,
        }
    }
}

impl From<ListGroupsResponseGroups3> for ListGroupsResponseGroups4 {
    fn from(older: ListGroupsResponseGroups3) -> Self {
        ListGroupsResponseGroups4 {
            group_id: older.group_id,
            protocol_type: older.protocol_type,
            tag_buffer: older.tag_buffer,
            ..ListGroupsResponseGroups4::default()
        }
    }
}
