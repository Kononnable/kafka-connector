use super::prelude::*;

pub type DeleteGroupsRequest = DeleteGroupsRequest2;
pub type DeleteGroupsResponse = DeleteGroupsResponse2;
impl ApiCall for DeleteGroupsRequest {
    type Response = DeleteGroupsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DeleteGroups
    }
    fn get_first_error(response: &DeleteGroupsResponse) -> Option<ApiError> {
        DeleteGroupsResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                DeleteGroupsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DeleteGroupsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &DeleteGroupsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &DeleteGroupsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DeleteGroupsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => DeleteGroupsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => DeleteGroupsResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => DeleteGroupsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => DeleteGroupsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteGroupsRequest0 {
    pub groups_names: Vec<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteGroupsRequest1 {
    pub groups_names: Vec<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteGroupsRequest2 {
    pub groups_names: Vec<String>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteGroupsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DeleteGroupsResponseResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteGroupsResponseResults0 {
    pub group_id: String,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteGroupsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DeleteGroupsResponseResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteGroupsResponseResults1 {
    pub group_id: String,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteGroupsResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DeleteGroupsResponseResults2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteGroupsResponseResults2 {
    pub group_id: String,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<DeleteGroupsRequest2> for DeleteGroupsRequest0 {
    fn from(latest: DeleteGroupsRequest2) -> DeleteGroupsRequest0 {
        DeleteGroupsRequest0 {
            groups_names: latest.groups_names,
        }
    }
}

impl From<DeleteGroupsRequest2> for DeleteGroupsRequest1 {
    fn from(latest: DeleteGroupsRequest2) -> DeleteGroupsRequest1 {
        DeleteGroupsRequest1 {
            groups_names: latest.groups_names,
        }
    }
}

impl From<DeleteGroupsResponse0> for DeleteGroupsResponse2 {
    fn from(older: DeleteGroupsResponse0) -> Self {
        DeleteGroupsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
            ..DeleteGroupsResponse2::default()
        }
    }
}

impl From<DeleteGroupsResponseResults0> for DeleteGroupsResponseResults2 {
    fn from(older: DeleteGroupsResponseResults0) -> Self {
        DeleteGroupsResponseResults2 {
            group_id: older.group_id,
            error_code: older.error_code,
            ..DeleteGroupsResponseResults2::default()
        }
    }
}

impl From<DeleteGroupsResponse1> for DeleteGroupsResponse2 {
    fn from(older: DeleteGroupsResponse1) -> Self {
        DeleteGroupsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
            ..DeleteGroupsResponse2::default()
        }
    }
}

impl From<DeleteGroupsResponseResults1> for DeleteGroupsResponseResults2 {
    fn from(older: DeleteGroupsResponseResults1) -> Self {
        DeleteGroupsResponseResults2 {
            group_id: older.group_id,
            error_code: older.error_code,
            ..DeleteGroupsResponseResults2::default()
        }
    }
}

impl DeleteGroupsResponse2 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DeleteGroupsResponseResults2 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
