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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&ListGroupsRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&ListGroupsRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&ListGroupsRequest2::try_from(self)?, buf),
            3 => ToBytes::serialize(&ListGroupsRequest3::try_from(self)?, buf),
            4 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> ListGroupsResponse {
        match version {
            0 => ListGroupsResponse0::deserialize(buf).into(),
            1 => ListGroupsResponse1::deserialize(buf).into(),
            2 => ListGroupsResponse2::deserialize(buf).into(),
            3 => ListGroupsResponse3::deserialize(buf).into(),
            4 => ListGroupsResponse::deserialize(buf),
            _ => ListGroupsResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, ToBytes)]
pub struct ListGroupsRequest0 {}

#[derive(Default, Debug, ToBytes)]
pub struct ListGroupsRequest1 {}

#[derive(Default, Debug, ToBytes)]
pub struct ListGroupsRequest2 {}

#[derive(Default, Debug, ToBytes)]
pub struct ListGroupsRequest3 {}

#[derive(Default, Debug, ToBytes)]
pub struct ListGroupsRequest4 {
    pub states_filter: Optional<Vec<CompactString>>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponse0 {
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponseGroups0 {
    pub group_id: String,
    pub protocol_type: String,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponseGroups1 {
    pub group_id: String,
    pub protocol_type: String,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups2>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponseGroups2 {
    pub group_id: String,
    pub protocol_type: String,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups3>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponseGroups3 {
    pub group_id: CompactString,
    pub protocol_type: CompactString,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub groups: Vec<ListGroupsResponseGroups4>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ListGroupsResponseGroups4 {
    pub group_id: CompactString,
    pub protocol_type: CompactString,
    pub group_state: Optional<CompactString>,
}

impl TryFrom<ListGroupsRequest4> for ListGroupsRequest0 {
    type Error = Error;
    fn try_from(latest: ListGroupsRequest4) -> Result<Self, Self::Error> {
        if latest.states_filter.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListGroupsRequest",
                0,
                "states_filter",
            ));
        }
        Ok(ListGroupsRequest0 {})
    }
}

impl TryFrom<ListGroupsRequest4> for ListGroupsRequest1 {
    type Error = Error;
    fn try_from(latest: ListGroupsRequest4) -> Result<Self, Self::Error> {
        if latest.states_filter.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListGroupsRequest",
                1,
                "states_filter",
            ));
        }
        Ok(ListGroupsRequest1 {})
    }
}

impl TryFrom<ListGroupsRequest4> for ListGroupsRequest2 {
    type Error = Error;
    fn try_from(latest: ListGroupsRequest4) -> Result<Self, Self::Error> {
        if latest.states_filter.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListGroupsRequest",
                2,
                "states_filter",
            ));
        }
        Ok(ListGroupsRequest2 {})
    }
}

impl TryFrom<ListGroupsRequest4> for ListGroupsRequest3 {
    type Error = Error;
    fn try_from(latest: ListGroupsRequest4) -> Result<Self, Self::Error> {
        if latest.states_filter.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListGroupsRequest",
                3,
                "states_filter",
            ));
        }
        Ok(ListGroupsRequest3 {})
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
            group_id: older.group_id.into(),
            protocol_type: older.protocol_type.into(),
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
        }
    }
}

impl From<ListGroupsResponseGroups1> for ListGroupsResponseGroups4 {
    fn from(older: ListGroupsResponseGroups1) -> Self {
        ListGroupsResponseGroups4 {
            group_id: older.group_id.into(),
            protocol_type: older.protocol_type.into(),
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
        }
    }
}

impl From<ListGroupsResponseGroups2> for ListGroupsResponseGroups4 {
    fn from(older: ListGroupsResponseGroups2) -> Self {
        ListGroupsResponseGroups4 {
            group_id: older.group_id.into(),
            protocol_type: older.protocol_type.into(),
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
        }
    }
}

impl From<ListGroupsResponseGroups3> for ListGroupsResponseGroups4 {
    fn from(older: ListGroupsResponseGroups3) -> Self {
        ListGroupsResponseGroups4 {
            group_id: older.group_id,
            protocol_type: older.protocol_type,
            ..ListGroupsResponseGroups4::default()
        }
    }
}
