use super::prelude::*;

pub type AlterConfigsRequest = AlterConfigsRequest1;
pub type AlterConfigsResponse = AlterConfigsResponse1;
pub fn serialize_alter_configs_request(
    data: AlterConfigsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&AlterConfigsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_alter_configs_response(version: i32, buf: &mut Bytes) -> AlterConfigsResponse {
    match version {
        0 => AlterConfigsResponse0::deserialize(buf).into(),
        1 => AlterConfigsResponse::deserialize(buf),
        _ => AlterConfigsResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterConfigsRequest0 {
    pub resources: Vec<AlterConfigsRequestResources0>,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterConfigsRequestResources0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<AlterConfigsRequestResourcesConfigs0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterConfigsRequestResourcesConfigs0 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterConfigsRequest1 {
    pub resources: Vec<AlterConfigsRequestResources1>,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterConfigsRequestResources1 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<AlterConfigsRequestResourcesConfigs1>,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterConfigsRequestResourcesConfigs1 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterConfigsResponse0 {
    pub throttle_time_ms: Int32,
    pub responses: Vec<AlterConfigsResponseResponses0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterConfigsResponseResponses0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterConfigsResponse1 {
    pub throttle_time_ms: Int32,
    pub responses: Vec<AlterConfigsResponseResponses1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterConfigsResponseResponses1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
}

impl TryFrom<AlterConfigsRequest1> for AlterConfigsRequest0 {
    type Error = Error;
    fn try_from(latest: AlterConfigsRequest1) -> Result<Self, Self::Error> {
        Ok(AlterConfigsRequest0 {
            resources: latest
                .resources
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<AlterConfigsRequestResources1> for AlterConfigsRequestResources0 {
    type Error = Error;
    fn try_from(latest: AlterConfigsRequestResources1) -> Result<Self, Self::Error> {
        Ok(AlterConfigsRequestResources0 {
            resource_type: latest.resource_type,
            resource_name: latest.resource_name,
            configs: latest
                .configs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<AlterConfigsRequestResourcesConfigs1> for AlterConfigsRequestResourcesConfigs0 {
    type Error = Error;
    fn try_from(latest: AlterConfigsRequestResourcesConfigs1) -> Result<Self, Self::Error> {
        Ok(AlterConfigsRequestResourcesConfigs0 {
            name: latest.name,
            value: latest.value,
        })
    }
}

impl From<AlterConfigsResponse0> for AlterConfigsResponse1 {
    fn from(older: AlterConfigsResponse0) -> Self {
        AlterConfigsResponse1 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<AlterConfigsResponseResponses0> for AlterConfigsResponseResponses1 {
    fn from(older: AlterConfigsResponseResponses0) -> Self {
        AlterConfigsResponseResponses1 {
            error_code: older.error_code,
            error_message: older.error_message,
            resource_type: older.resource_type,
            resource_name: older.resource_name,
        }
    }
}
