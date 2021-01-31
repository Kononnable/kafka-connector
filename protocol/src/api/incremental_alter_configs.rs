use super::prelude::*;

pub type IncrementalAlterConfigsRequest = IncrementalAlterConfigsRequest1;
pub type IncrementalAlterConfigsResponse = IncrementalAlterConfigsResponse1;
impl ApiCall for IncrementalAlterConfigsRequest {
    type Response = IncrementalAlterConfigsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        1
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::IncrementalAlterConfigs
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => true,
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
                IncrementalAlterConfigsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                IncrementalAlterConfigsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &IncrementalAlterConfigsRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(
        version: i16,
        buf: &mut Bytes,
    ) -> (i32, IncrementalAlterConfigsResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => IncrementalAlterConfigsResponse0::deserialize(
                buf,
                Self::is_flexible_version(version),
            )
            .into(),
            1 => IncrementalAlterConfigsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
            _ => IncrementalAlterConfigsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct IncrementalAlterConfigsRequest0 {
    pub resources: Vec<IncrementalAlterConfigsRequestResources0>,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct IncrementalAlterConfigsRequestResources0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<IncrementalAlterConfigsRequestResourcesConfigs0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct IncrementalAlterConfigsRequestResourcesConfigs0 {
    pub name: String,
    pub config_operation: Int8,
    pub value: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct IncrementalAlterConfigsRequest1 {
    pub resources: Vec<IncrementalAlterConfigsRequestResources1>,
    pub validate_only: Boolean,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct IncrementalAlterConfigsRequestResources1 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<IncrementalAlterConfigsRequestResourcesConfigs1>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct IncrementalAlterConfigsRequestResourcesConfigs1 {
    pub name: String,
    pub config_operation: Int8,
    pub value: NullableString,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct IncrementalAlterConfigsResponse0 {
    pub throttle_time_ms: Int32,
    pub responses: Vec<IncrementalAlterConfigsResponseResponses0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct IncrementalAlterConfigsResponseResponses0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct IncrementalAlterConfigsResponse1 {
    pub throttle_time_ms: Int32,
    pub responses: Vec<IncrementalAlterConfigsResponseResponses1>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct IncrementalAlterConfigsResponseResponses1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub tag_buffer: Optional<TagBuffer>,
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
            resource_name: latest.resource_name,
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
            name: latest.name,
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
            ..IncrementalAlterConfigsResponse1::default()
        }
    }
}

impl From<IncrementalAlterConfigsResponseResponses0> for IncrementalAlterConfigsResponseResponses1 {
    fn from(older: IncrementalAlterConfigsResponseResponses0) -> Self {
        IncrementalAlterConfigsResponseResponses1 {
            error_code: older.error_code,
            error_message: older.error_message,
            resource_type: older.resource_type,
            resource_name: older.resource_name,
            ..IncrementalAlterConfigsResponseResponses1::default()
        }
    }
}
