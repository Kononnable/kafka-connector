use super::prelude::*;

pub type DeleteGroupsRequest = DeleteGroupsRequest2;
pub type DeleteGroupsResponse = DeleteGroupsResponse2;
pub fn serialize_delete_groups_request(
    data: DeleteGroupsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&DeleteGroupsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&DeleteGroupsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_delete_groups_response(version: i32, buf: &mut Bytes) -> DeleteGroupsResponse {
    match version {
        0 => DeleteGroupsResponse0::deserialize(buf).into(),
        1 => DeleteGroupsResponse1::deserialize(buf).into(),
        2 => DeleteGroupsResponse::deserialize(buf),
        _ => DeleteGroupsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct DeleteGroupsRequest0 {
    pub groups_names: Vec<String>,
}

#[derive(Default, ToBytes)]
pub struct DeleteGroupsRequest1 {
    pub groups_names: Vec<String>,
}

#[derive(Default, ToBytes)]
pub struct DeleteGroupsRequest2 {
    pub groups_names: Vec<CompactString>,
}

#[derive(Default, FromBytes)]
pub struct DeleteGroupsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DeleteGroupsResponseResults0>,
}

#[derive(Default, FromBytes)]
pub struct DeleteGroupsResponseResults0 {
    pub group_id: String,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteGroupsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DeleteGroupsResponseResults1>,
}

#[derive(Default, FromBytes)]
pub struct DeleteGroupsResponseResults1 {
    pub group_id: String,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteGroupsResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DeleteGroupsResponseResults2>,
}

#[derive(Default, FromBytes)]
pub struct DeleteGroupsResponseResults2 {
    pub group_id: CompactString,
    pub error_code: Int16,
}

impl TryFrom<DeleteGroupsRequest2> for DeleteGroupsRequest0 {
    type Error = Error;
    fn try_from(latest: DeleteGroupsRequest2) -> Result<Self, Self::Error> {
        Ok(DeleteGroupsRequest0 {
            groups_names: latest.groups_names.into_iter().collect(),
        })
    }
}

impl TryFrom<DeleteGroupsRequest2> for DeleteGroupsRequest1 {
    type Error = Error;
    fn try_from(latest: DeleteGroupsRequest2) -> Result<Self, Self::Error> {
        Ok(DeleteGroupsRequest1 {
            groups_names: latest.groups_names.into_iter().collect(),
        })
    }
}

impl From<DeleteGroupsResponse0> for DeleteGroupsResponse2 {
    fn from(older: DeleteGroupsResponse0) -> Self {
        DeleteGroupsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DeleteGroupsResponseResults0> for DeleteGroupsResponseResults2 {
    fn from(older: DeleteGroupsResponseResults0) -> Self {
        DeleteGroupsResponseResults2 {
            group_id: older.group_id,
            error_code: older.error_code,
        }
    }
}

impl From<DeleteGroupsResponse1> for DeleteGroupsResponse2 {
    fn from(older: DeleteGroupsResponse1) -> Self {
        DeleteGroupsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DeleteGroupsResponseResults1> for DeleteGroupsResponseResults2 {
    fn from(older: DeleteGroupsResponseResults1) -> Self {
        DeleteGroupsResponseResults2 {
            group_id: older.group_id,
            error_code: older.error_code,
        }
    }
}
