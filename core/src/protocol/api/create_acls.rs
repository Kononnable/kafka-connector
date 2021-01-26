use super::prelude::*;

pub type CreateAclsRequest = CreateAclsRequest2;
pub type CreateAclsResponse = CreateAclsResponse2;
pub fn serialize_create_acls_request(
    data: CreateAclsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&CreateAclsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&CreateAclsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_create_acls_response(version: i32, buf: &mut Bytes) -> CreateAclsResponse {
    match version {
        0 => CreateAclsResponse0::deserialize(buf).into(),
        1 => CreateAclsResponse1::deserialize(buf).into(),
        2 => CreateAclsResponse::deserialize(buf),
        _ => CreateAclsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct CreateAclsRequest0 {
    pub creations: Vec<CreateAclsRequestCreations0>,
}

#[derive(Default, ToBytes)]
pub struct CreateAclsRequestCreations0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, ToBytes)]
pub struct CreateAclsRequest1 {
    pub creations: Vec<CreateAclsRequestCreations1>,
}

#[derive(Default, ToBytes)]
pub struct CreateAclsRequestCreations1 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub resource_pattern_type: Optional<Int8>,
    pub principal: String,
    pub host: String,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, ToBytes)]
pub struct CreateAclsRequest2 {
    pub creations: Vec<CreateAclsRequestCreations2>,
}

#[derive(Default, ToBytes)]
pub struct CreateAclsRequestCreations2 {
    pub resource_type: Int8,
    pub resource_name: CompactString,
    pub resource_pattern_type: Optional<Int8>,
    pub principal: CompactString,
    pub host: CompactString,
    pub operation: Int8,
    pub permission_type: Int8,
}

#[derive(Default, FromBytes)]
pub struct CreateAclsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreateAclsResponseResults0>,
}

#[derive(Default, FromBytes)]
pub struct CreateAclsResponseResults0 {
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, FromBytes)]
pub struct CreateAclsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreateAclsResponseResults1>,
}

#[derive(Default, FromBytes)]
pub struct CreateAclsResponseResults1 {
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, FromBytes)]
pub struct CreateAclsResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreateAclsResponseResults2>,
}

#[derive(Default, FromBytes)]
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
                .map(|el| el.try_into())
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
            resource_name: latest.resource_name,
            principal: latest.principal,
            host: latest.host,
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
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateAclsRequestCreations2> for CreateAclsRequestCreations1 {
    type Error = Error;
    fn try_from(latest: CreateAclsRequestCreations2) -> Result<Self, Self::Error> {
        Ok(CreateAclsRequestCreations1 {
            resource_type: latest.resource_type,
            resource_name: latest.resource_name,
            resource_pattern_type: latest.resource_pattern_type,
            principal: latest.principal,
            host: latest.host,
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
            error_message: older.error_message,
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
            error_message: older.error_message,
        }
    }
}
