use super::prelude::*;

pub type IncrementalAlterConfigsRequest = IncrementalAlterConfigsRequest1;
pub type IncrementalAlterConfigsResponse = IncrementalAlterConfigsResponse1;
pub fn serialize_incremental_alter_configs_request(
    data: IncrementalAlterConfigsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&IncrementalAlterConfigsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_incremental_alter_configs_response(
    version: i32,
    buf: &mut Bytes,
) -> IncrementalAlterConfigsResponse {
    match version {
        0 => IncrementalAlterConfigsResponse0::deserialize(buf).into(),
        1 => IncrementalAlterConfigsResponse::deserialize(buf),
        _ => IncrementalAlterConfigsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct IncrementalAlterConfigsRequest0 {
    pub resources: Vec<IncrementalAlterConfigsRequestResources0>,
    pub validate_only: Boolean,
}

#[derive(Default, ToBytes)]
pub struct IncrementalAlterConfigsRequestResources0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<IncrementalAlterConfigsRequestResourcesConfigs0>,
}

#[derive(Default, ToBytes)]
pub struct IncrementalAlterConfigsRequestResourcesConfigs0 {
    pub name: String,
    pub config_operation: Int8,
    pub value: NullableString,
}

#[derive(Default, ToBytes)]
pub struct IncrementalAlterConfigsRequest1 {
    pub resources: Vec<IncrementalAlterConfigsRequestResources1>,
    pub validate_only: Boolean,
}

#[derive(Default, ToBytes)]
pub struct IncrementalAlterConfigsRequestResources1 {
    pub resource_type: Int8,
    pub resource_name: CompactString,
    pub configs: Vec<IncrementalAlterConfigsRequestResourcesConfigs1>,
}

#[derive(Default, ToBytes)]
pub struct IncrementalAlterConfigsRequestResourcesConfigs1 {
    pub name: CompactString,
    pub config_operation: Int8,
    pub value: CompactNullableString,
}

#[derive(Default, FromBytes)]
pub struct IncrementalAlterConfigsResponse0 {
    pub throttle_time_ms: Int32,
    pub responses: Vec<IncrementalAlterConfigsResponseResponses0>,
}

#[derive(Default, FromBytes)]
pub struct IncrementalAlterConfigsResponseResponses0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
}

#[derive(Default, FromBytes)]
pub struct IncrementalAlterConfigsResponse1 {
    pub throttle_time_ms: Int32,
    pub responses: Vec<IncrementalAlterConfigsResponseResponses1>,
}

#[derive(Default, FromBytes)]
pub struct IncrementalAlterConfigsResponseResponses1 {
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub resource_type: Int8,
    pub resource_name: CompactString,
}

impl TryFrom<IncrementalAlterConfigsRequest1> for IncrementalAlterConfigsRequest0 {
    type Error = Error;
    fn try_from(latest: IncrementalAlterConfigsRequest1) -> Result<Self, Self::Error> {
        Ok(IncrementalAlterConfigsRequest0 {
            resources: latest
                .resources
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<IncrementalAlterConfigsRequestResources1>
    for IncrementalAlterConfigsRequestResources0
{
    type Error = Error;
    fn try_from(latest: IncrementalAlterConfigsRequestResources1) -> Result<Self, Self::Error> {
        Ok(IncrementalAlterConfigsRequestResources0 {
            resource_type: latest.resource_type,
            resource_name: latest.resource_name.into(),
            configs: latest
                .configs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<IncrementalAlterConfigsRequestResourcesConfigs1>
    for IncrementalAlterConfigsRequestResourcesConfigs0
{
    type Error = Error;
    fn try_from(
        latest: IncrementalAlterConfigsRequestResourcesConfigs1,
    ) -> Result<Self, Self::Error> {
        Ok(IncrementalAlterConfigsRequestResourcesConfigs0 {
            name: latest.name.into(),
            config_operation: latest.config_operation,
            value: latest.value,
        })
    }
}

impl From<IncrementalAlterConfigsResponse0> for IncrementalAlterConfigsResponse1 {
    fn from(older: IncrementalAlterConfigsResponse0) -> Self {
        IncrementalAlterConfigsResponse1 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<IncrementalAlterConfigsResponseResponses0> for IncrementalAlterConfigsResponseResponses1 {
    fn from(older: IncrementalAlterConfigsResponseResponses0) -> Self {
        IncrementalAlterConfigsResponseResponses1 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
            resource_type: older.resource_type,
            resource_name: older.resource_name.into(),
        }
    }
}
