use super::prelude::*;

pub type AlterConfigsRequest = AlterConfigsRequest1;
pub type AlterConfigsResponse = AlterConfigsResponse1;
impl ApiCall for AlterConfigsRequest {
    type Response = AlterConfigsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        1
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterConfigs
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            _ => false,
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
                AlterConfigsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                AlterConfigsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &AlterConfigsRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, AlterConfigsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => AlterConfigsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => AlterConfigsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => AlterConfigsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterConfigsRequest0 {
    pub resources: Vec<AlterConfigsRequestResources0>,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterConfigsRequestResources0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<AlterConfigsRequestResourcesConfigs0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterConfigsRequestResourcesConfigs0 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterConfigsRequest1 {
    pub resources: Vec<AlterConfigsRequestResources1>,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterConfigsRequestResources1 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<AlterConfigsRequestResourcesConfigs1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterConfigsRequestResourcesConfigs1 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterConfigsResponse0 {
    pub throttle_time_ms: Int32,
    pub responses: Vec<AlterConfigsResponseResponses0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterConfigsResponseResponses0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterConfigsResponse1 {
    pub throttle_time_ms: Int32,
    pub responses: Vec<AlterConfigsResponseResponses1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
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
