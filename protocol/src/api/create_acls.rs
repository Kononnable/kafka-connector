use super::prelude::*;

pub type CreateAclsRequest = CreateAclsRequest2;
pub type CreateAclsResponse = CreateAclsResponse2;
impl ApiCall for CreateAclsRequest {
    type Response = CreateAclsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::CreateAcls
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&CreateAclsRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&CreateAclsRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> CreateAclsResponse {
        match version {
            0 => CreateAclsResponse0::deserialize(buf).into(),
            1 => CreateAclsResponse1::deserialize(buf).into(),
            2 => CreateAclsResponse::deserialize(buf),
            _ => CreateAclsResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, ToBytes)]
pub struct CreateAclsRequest0 {
    pub creations: Vec<CreateAclsRequestCreations0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateAclsRequestCreations0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateAclsRequest1 {
    pub creations: Vec<CreateAclsRequestCreations1>,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateAclsRequestCreations1 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub resource_pattern_type: Optional<Int8>,
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateAclsRequest2 {
    pub creations: Vec<CreateAclsRequestCreations2>,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateAclsRequestCreations2 {
    pub resource_type: Int8,
    pub resource_name: CompactString,
    pub resource_pattern_type: Optional<Int8>,
    pub principal: CompactString,
    pub host: CompactString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateAclsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreateAclsResponseResults0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateAclsResponseResults0 {
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateAclsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreateAclsResponseResults1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateAclsResponseResults1 {
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateAclsResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreateAclsResponseResults2>,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateAclsResponseResults2 {
    pub error_code: Int16,
    pub error_message: CompactNullableString,
}

impl TryFrom<CreateAclsRequest2> for CreateAclsRequest0 {
    type Error = Error;
    fn try_from(latest: CreateAclsRequest2) -> Result<Self, Self::Error> {
        Ok(CreateAclsRequest0 {
            creations: latest
                .creations
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateAclsRequestCreations2> for CreateAclsRequestCreations0 {
    type Error = Error;
    fn try_from(latest: CreateAclsRequestCreations2) -> Result<Self, Self::Error> {
        if latest.resource_pattern_type.is_some() {
            return Err(Error::OldKafkaVersion(
                "CreateAclsRequestCreations",
                0,
                "resource_pattern_type",
            ));
        }
        Ok(CreateAclsRequestCreations0 {
            resource_type: latest.resource_type,
            resource_name: latest.resource_name.into(),
            principal: latest.principal.into(),
            host: latest.host.into(),
            operation: latest.operation,
            permission_type: latest.permission_type,
        })
    }
}

impl TryFrom<CreateAclsRequest2> for CreateAclsRequest1 {
    type Error = Error;
    fn try_from(latest: CreateAclsRequest2) -> Result<Self, Self::Error> {
        Ok(CreateAclsRequest1 {
            creations: latest
                .creations
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateAclsRequestCreations2> for CreateAclsRequestCreations1 {
    type Error = Error;
    fn try_from(latest: CreateAclsRequestCreations2) -> Result<Self, Self::Error> {
        Ok(CreateAclsRequestCreations1 {
            resource_type: latest.resource_type,
            resource_name: latest.resource_name.into(),
            resource_pattern_type: latest.resource_pattern_type,
            principal: latest.principal.into(),
            host: latest.host.into(),
            operation: latest.operation,
            permission_type: latest.permission_type,
        })
    }
}

impl From<CreateAclsResponse0> for CreateAclsResponse2 {
    fn from(older: CreateAclsResponse0) -> Self {
        CreateAclsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreateAclsResponseResults0> for CreateAclsResponseResults2 {
    fn from(older: CreateAclsResponseResults0) -> Self {
        CreateAclsResponseResults2 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
        }
    }
}

impl From<CreateAclsResponse1> for CreateAclsResponse2 {
    fn from(older: CreateAclsResponse1) -> Self {
        CreateAclsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreateAclsResponseResults1> for CreateAclsResponseResults2 {
    fn from(older: CreateAclsResponseResults1) -> Self {
        CreateAclsResponseResults2 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
        }
    }
}