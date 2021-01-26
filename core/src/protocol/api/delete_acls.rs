use super::prelude::*;

pub type DeleteAclsRequest = DeleteAclsRequest2;
pub type DeleteAclsResponse = DeleteAclsResponse2;
pub fn serialize_delete_acls_request(
    data: DeleteAclsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&DeleteAclsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&DeleteAclsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_delete_acls_response(version: i32, buf: &mut Bytes) -> DeleteAclsResponse {
    match version {
        0 => DeleteAclsResponse0::deserialize(buf).into(),
        1 => DeleteAclsResponse1::deserialize(buf).into(),
        2 => DeleteAclsResponse::deserialize(buf),
        _ => DeleteAclsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct DeleteAclsRequest0 {
    pub filters: Vec<DeleteAclsRequestFilters0>,
}

#[derive(Default, ToBytes)]
pub struct DeleteAclsRequestFilters0 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, ToBytes)]
pub struct DeleteAclsRequest1 {
    pub filters: Vec<DeleteAclsRequestFilters1>,
}

#[derive(Default, ToBytes)]
pub struct DeleteAclsRequestFilters1 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: NullableString,
    pub pattern_type_filter: Optional<Int8>,
    pub principal_filter: NullableString,
    pub host_filter: NullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, ToBytes)]
pub struct DeleteAclsRequest2 {
    pub filters: Vec<DeleteAclsRequestFilters2>,
}

#[derive(Default, ToBytes)]
pub struct DeleteAclsRequestFilters2 {
    pub resource_type_filter: Int8,
    pub resource_name_filter: CompactNullableString,
    pub pattern_type_filter: Optional<Int8>,
    pub principal_filter: CompactNullableString,
    pub host_filter: CompactNullableString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, FromBytes)]
pub struct DeleteAclsResponse0 {
    pub throttle_time_ms: Int32,
    pub filter_results: Vec<DeleteAclsResponseFilterResults0>,
}

#[derive(Default, FromBytes)]
pub struct DeleteAclsResponseFilterResults0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub matching_acls: Vec<DeleteAclsResponseFilterResultsMatchingAcls0>,
}

#[derive(Default, FromBytes)]
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

#[derive(Default, FromBytes)]
pub struct DeleteAclsResponse1 {
    pub throttle_time_ms: Int32,
    pub filter_results: Vec<DeleteAclsResponseFilterResults1>,
}

#[derive(Default, FromBytes)]
pub struct DeleteAclsResponseFilterResults1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub matching_acls: Vec<DeleteAclsResponseFilterResultsMatchingAcls1>,
}

#[derive(Default, FromBytes)]
pub struct DeleteAclsResponseFilterResultsMatchingAcls1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub pattern_type: Optional<Int8>,
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, FromBytes)]
pub struct DeleteAclsResponse2 {
    pub throttle_time_ms: Int32,
    pub filter_results: Vec<DeleteAclsResponseFilterResults2>,
}

#[derive(Default, FromBytes)]
pub struct DeleteAclsResponseFilterResults2 {
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub matching_acls: Vec<DeleteAclsResponseFilterResultsMatchingAcls2>,
}

#[derive(Default, FromBytes)]
pub struct DeleteAclsResponseFilterResultsMatchingAcls2 {
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub resource_type: Int8,
    pub resource_name: CompactString,
    pub pattern_type: Optional<Int8>,
    pub principal: CompactString,
    pub host: CompactString,
    pub operation: Int8,
    pub permission_type: Int8,
}

impl TryFrom<DeleteAclsRequest2> for DeleteAclsRequest0 {
    type Error = Error;
    fn try_from(latest: DeleteAclsRequest2) -> Result<Self, Self::Error> {
        Ok(DeleteAclsRequest0 {
            filters: latest
                .filters
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DeleteAclsRequestFilters2> for DeleteAclsRequestFilters0 {
    type Error = Error;
    fn try_from(latest: DeleteAclsRequestFilters2) -> Result<Self, Self::Error> {
        if latest.pattern_type_filter.is_some() {
            return Err(Error::OldKafkaVersion(
                "DeleteAclsRequestFilters",
                0,
                "pattern_type_filter",
            ));
        }
        Ok(DeleteAclsRequestFilters0 {
            resource_type_filter: latest.resource_type_filter,
            resource_name_filter: latest.resource_name_filter,
            principal_filter: latest.principal_filter,
            host_filter: latest.host_filter,
            operation: latest.operation,
            permission_type: latest.permission_type,
        })
    }
}

impl TryFrom<DeleteAclsRequest2> for DeleteAclsRequest1 {
    type Error = Error;
    fn try_from(latest: DeleteAclsRequest2) -> Result<Self, Self::Error> {
        Ok(DeleteAclsRequest1 {
            filters: latest
                .filters
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DeleteAclsRequestFilters2> for DeleteAclsRequestFilters1 {
    type Error = Error;
    fn try_from(latest: DeleteAclsRequestFilters2) -> Result<Self, Self::Error> {
        Ok(DeleteAclsRequestFilters1 {
            resource_type_filter: latest.resource_type_filter,
            resource_name_filter: latest.resource_name_filter,
            pattern_type_filter: latest.pattern_type_filter,
            principal_filter: latest.principal_filter,
            host_filter: latest.host_filter,
            operation: latest.operation,
            permission_type: latest.permission_type,
        })
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
        }
    }
}

impl From<DeleteAclsResponseFilterResults0> for DeleteAclsResponseFilterResults2 {
    fn from(older: DeleteAclsResponseFilterResults0) -> Self {
        DeleteAclsResponseFilterResults2 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
            matching_acls: older
                .matching_acls
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<DeleteAclsResponseFilterResultsMatchingAcls0>
    for DeleteAclsResponseFilterResultsMatchingAcls2
{
    fn from(older: DeleteAclsResponseFilterResultsMatchingAcls0) -> Self {
        DeleteAclsResponseFilterResultsMatchingAcls2 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
            resource_type: older.resource_type,
            resource_name: older.resource_name.into(),
            principal: older.principal.into(),
            host: older.host.into(),
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
        }
    }
}

impl From<DeleteAclsResponseFilterResults1> for DeleteAclsResponseFilterResults2 {
    fn from(older: DeleteAclsResponseFilterResults1) -> Self {
        DeleteAclsResponseFilterResults2 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
            matching_acls: older
                .matching_acls
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<DeleteAclsResponseFilterResultsMatchingAcls1>
    for DeleteAclsResponseFilterResultsMatchingAcls2
{
    fn from(older: DeleteAclsResponseFilterResultsMatchingAcls1) -> Self {
        DeleteAclsResponseFilterResultsMatchingAcls2 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
            resource_type: older.resource_type,
            resource_name: older.resource_name.into(),
            pattern_type: older.pattern_type,
            principal: older.principal.into(),
            host: older.host.into(),
            operation: older.operation,
            permission_type: older.permission_type,
        }
    }
}
