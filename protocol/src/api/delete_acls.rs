use super::prelude::*;

pub type DeleteAclsRequest = DeleteAclsRequest2;
pub type DeleteAclsResponse = DeleteAclsResponse2;
impl ApiCall for DeleteAclsRequest {
    type Response = DeleteAclsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DeleteAcls
    }
    fn get_first_error(response: &DeleteAclsResponse) -> Option<ApiError> {
        DeleteAclsResponse::get_first_error(response)
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
                DeleteAclsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DeleteAclsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &DeleteAclsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &DeleteAclsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DeleteAclsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => DeleteAclsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => DeleteAclsResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => DeleteAclsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => DeleteAclsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteAclsRequest0 {
    pub filters: Vec<DeleteAclsRequestFilters0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteAclsRequestFilters0 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteAclsRequest1 {
    pub filters: Vec<DeleteAclsRequestFilters1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteAclsRequestFilters1 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub pattern_type_filter: Int8,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteAclsRequest2 {
    pub filters: Vec<DeleteAclsRequestFilters2>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteAclsRequestFilters2 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub pattern_type_filter: Int8,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponse0 {
    pub throttle_time_ms: Int32,
    pub filter_results: Vec<DeleteAclsResponseFilterResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponseFilterResults0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub matching_acls: Vec<DeleteAclsResponseFilterResultsMatchingAcls0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponseFilterResultsMatchingAcls0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponse1 {
    pub throttle_time_ms: Int32,
    pub filter_results: Vec<DeleteAclsResponseFilterResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponseFilterResults1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub matching_acls: Vec<DeleteAclsResponseFilterResultsMatchingAcls1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponseFilterResultsMatchingAcls1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub pattern_type: Option<Int8>,
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponse2 {
    pub throttle_time_ms: Int32,
    pub filter_results: Vec<DeleteAclsResponseFilterResults2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponseFilterResults2 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub matching_acls: Vec<DeleteAclsResponseFilterResultsMatchingAcls2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteAclsResponseFilterResultsMatchingAcls2 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub pattern_type: Option<Int8>,
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<DeleteAclsRequest2> for DeleteAclsRequest0 {
    fn from(latest: DeleteAclsRequest2) -> DeleteAclsRequest0 {
        DeleteAclsRequest0 {
            filters: latest.filters.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<DeleteAclsRequestFilters2> for DeleteAclsRequestFilters0 {
    fn from(latest: DeleteAclsRequestFilters2) -> DeleteAclsRequestFilters0 {
        log::debug!(
            "Using old api format - DeleteAclsRequestFilters0, ignoring field pattern_type_filter"
        );
        DeleteAclsRequestFilters0 {
            resource_type_filter: latest.resource_type_filter,
            resource_name_filter: latest.resource_name_filter,
            principal_filter: latest.principal_filter,
            host_filter: latest.host_filter,
            operation: latest.operation,
            permission_type: latest.permission_type,
        }
    }
}

impl From<DeleteAclsRequest2> for DeleteAclsRequest1 {
    fn from(latest: DeleteAclsRequest2) -> DeleteAclsRequest1 {
        DeleteAclsRequest1 {
            filters: latest.filters.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<DeleteAclsRequestFilters2> for DeleteAclsRequestFilters1 {
    fn from(latest: DeleteAclsRequestFilters2) -> DeleteAclsRequestFilters1 {
        DeleteAclsRequestFilters1 {
            resource_type_filter: latest.resource_type_filter,
            resource_name_filter: latest.resource_name_filter,
            pattern_type_filter: latest.pattern_type_filter,
            principal_filter: latest.principal_filter,
            host_filter: latest.host_filter,
            operation: latest.operation,
            permission_type: latest.permission_type,
        }
    }
}

impl From<DeleteAclsResponse0> for DeleteAclsResponse2 {
    fn from(older: DeleteAclsResponse0) -> Self {
        DeleteAclsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            filter_results: older
                .filter_results
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..DeleteAclsResponse2::default()
        }
    }
}

impl From<DeleteAclsResponseFilterResults0> for DeleteAclsResponseFilterResults2 {
    fn from(older: DeleteAclsResponseFilterResults0) -> Self {
        DeleteAclsResponseFilterResults2 {
            error_code: older.error_code,
            error_message: older.error_message,
            matching_acls: older
                .matching_acls
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..DeleteAclsResponseFilterResults2::default()
        }
    }
}

impl From<DeleteAclsResponseFilterResultsMatchingAcls0>
    for DeleteAclsResponseFilterResultsMatchingAcls2
{
    fn from(older: DeleteAclsResponseFilterResultsMatchingAcls0) -> Self {
        DeleteAclsResponseFilterResultsMatchingAcls2 {
            error_code: older.error_code,
            error_message: older.error_message,
            resource_type: older.resource_type,
            resource_name: older.resource_name,
            principal: older.principal,
            host: older.host,
            operation: older.operation,
            permission_type: older.permission_type,
            ..DeleteAclsResponseFilterResultsMatchingAcls2::default()
        }
    }
}

impl From<DeleteAclsResponse1> for DeleteAclsResponse2 {
    fn from(older: DeleteAclsResponse1) -> Self {
        DeleteAclsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            filter_results: older
                .filter_results
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..DeleteAclsResponse2::default()
        }
    }
}

impl From<DeleteAclsResponseFilterResults1> for DeleteAclsResponseFilterResults2 {
    fn from(older: DeleteAclsResponseFilterResults1) -> Self {
        DeleteAclsResponseFilterResults2 {
            error_code: older.error_code,
            error_message: older.error_message,
            matching_acls: older
                .matching_acls
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..DeleteAclsResponseFilterResults2::default()
        }
    }
}

impl From<DeleteAclsResponseFilterResultsMatchingAcls1>
    for DeleteAclsResponseFilterResultsMatchingAcls2
{
    fn from(older: DeleteAclsResponseFilterResultsMatchingAcls1) -> Self {
        DeleteAclsResponseFilterResultsMatchingAcls2 {
            error_code: older.error_code,
            error_message: older.error_message,
            resource_type: older.resource_type,
            resource_name: older.resource_name,
            pattern_type: older.pattern_type,
            principal: older.principal,
            host: older.host,
            operation: older.operation,
            permission_type: older.permission_type,
            ..DeleteAclsResponseFilterResultsMatchingAcls2::default()
        }
    }
}

impl DeleteAclsResponse2 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.filter_results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DeleteAclsResponseFilterResults2 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.matching_acls.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DeleteAclsResponseFilterResultsMatchingAcls2 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
