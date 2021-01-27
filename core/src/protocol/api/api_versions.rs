use super::prelude::*;

pub type ApiVersionsRequest = ApiVersionsRequest3;
pub type ApiVersionsResponse = ApiVersionsResponse3;
pub fn serialize_api_versions_request(
    data: ApiVersionsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&ApiVersionsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&ApiVersionsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&ApiVersionsRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_api_versions_response(version: i32, buf: &mut Bytes) -> ApiVersionsResponse {
    match version {
        0 => ApiVersionsResponse0::deserialize(buf).into(),
        1 => ApiVersionsResponse1::deserialize(buf).into(),
        2 => ApiVersionsResponse2::deserialize(buf).into(),
        3 => ApiVersionsResponse::deserialize(buf),
        _ => ApiVersionsResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct ApiVersionsRequest0 {}

#[derive(Default, Debug, ToBytes)]
pub struct ApiVersionsRequest1 {}

#[derive(Default, Debug, ToBytes)]
pub struct ApiVersionsRequest2 {}

#[derive(Default, Debug, ToBytes)]
pub struct ApiVersionsRequest3 {
    pub client_software_name: Optional<CompactString>,
    pub client_software_version: Optional<CompactString>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ApiVersionsResponse0 {
    pub error_code: Int16,
    pub api_keys: Vec<ApiVersionsResponseApiKeys0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ApiVersionsResponseApiKeys0 {
    pub api_key: Int16,
    pub min_version: Int16,
    pub max_version: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct ApiVersionsResponse1 {
    pub error_code: Int16,
    pub api_keys: Vec<ApiVersionsResponseApiKeys1>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ApiVersionsResponseApiKeys1 {
    pub api_key: Int16,
    pub min_version: Int16,
    pub max_version: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct ApiVersionsResponse2 {
    pub error_code: Int16,
    pub api_keys: Vec<ApiVersionsResponseApiKeys2>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ApiVersionsResponseApiKeys2 {
    pub api_key: Int16,
    pub min_version: Int16,
    pub max_version: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct ApiVersionsResponse3 {
    pub error_code: Int16,
    pub api_keys: Vec<ApiVersionsResponseApiKeys3>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ApiVersionsResponseApiKeys3 {
    pub api_key: Int16,
    pub min_version: Int16,
    pub max_version: Int16,
}

impl TryFrom<ApiVersionsRequest3> for ApiVersionsRequest0 {
    type Error = Error;
    fn try_from(latest: ApiVersionsRequest3) -> Result<Self, Self::Error> {
        if latest.client_software_name.is_some() {
            return Err(Error::OldKafkaVersion(
                "ApiVersionsRequest",
                0,
                "client_software_name",
            ));
        }
        if latest.client_software_version.is_some() {
            return Err(Error::OldKafkaVersion(
                "ApiVersionsRequest",
                0,
                "client_software_version",
            ));
        }
        Ok(ApiVersionsRequest0 {})
    }
}

impl TryFrom<ApiVersionsRequest3> for ApiVersionsRequest1 {
    type Error = Error;
    fn try_from(latest: ApiVersionsRequest3) -> Result<Self, Self::Error> {
        if latest.client_software_name.is_some() {
            return Err(Error::OldKafkaVersion(
                "ApiVersionsRequest",
                1,
                "client_software_name",
            ));
        }
        if latest.client_software_version.is_some() {
            return Err(Error::OldKafkaVersion(
                "ApiVersionsRequest",
                1,
                "client_software_version",
            ));
        }
        Ok(ApiVersionsRequest1 {})
    }
}

impl TryFrom<ApiVersionsRequest3> for ApiVersionsRequest2 {
    type Error = Error;
    fn try_from(latest: ApiVersionsRequest3) -> Result<Self, Self::Error> {
        if latest.client_software_name.is_some() {
            return Err(Error::OldKafkaVersion(
                "ApiVersionsRequest",
                2,
                "client_software_name",
            ));
        }
        if latest.client_software_version.is_some() {
            return Err(Error::OldKafkaVersion(
                "ApiVersionsRequest",
                2,
                "client_software_version",
            ));
        }
        Ok(ApiVersionsRequest2 {})
    }
}

impl From<ApiVersionsResponse0> for ApiVersionsResponse3 {
    fn from(older: ApiVersionsResponse0) -> Self {
        ApiVersionsResponse3 {
            error_code: older.error_code,
            api_keys: older.api_keys.into_iter().map(|el| el.into()).collect(),
            ..ApiVersionsResponse3::default()
        }
    }
}

impl From<ApiVersionsResponseApiKeys0> for ApiVersionsResponseApiKeys3 {
    fn from(older: ApiVersionsResponseApiKeys0) -> Self {
        ApiVersionsResponseApiKeys3 {
            api_key: older.api_key,
            min_version: older.min_version,
            max_version: older.max_version,
        }
    }
}

impl From<ApiVersionsResponse1> for ApiVersionsResponse3 {
    fn from(older: ApiVersionsResponse1) -> Self {
        ApiVersionsResponse3 {
            error_code: older.error_code,
            api_keys: older.api_keys.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ApiVersionsResponseApiKeys1> for ApiVersionsResponseApiKeys3 {
    fn from(older: ApiVersionsResponseApiKeys1) -> Self {
        ApiVersionsResponseApiKeys3 {
            api_key: older.api_key,
            min_version: older.min_version,
            max_version: older.max_version,
        }
    }
}

impl From<ApiVersionsResponse2> for ApiVersionsResponse3 {
    fn from(older: ApiVersionsResponse2) -> Self {
        ApiVersionsResponse3 {
            error_code: older.error_code,
            api_keys: older.api_keys.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ApiVersionsResponseApiKeys2> for ApiVersionsResponseApiKeys3 {
    fn from(older: ApiVersionsResponseApiKeys2) -> Self {
        ApiVersionsResponseApiKeys3 {
            api_key: older.api_key,
            min_version: older.min_version,
            max_version: older.max_version,
        }
    }
}
