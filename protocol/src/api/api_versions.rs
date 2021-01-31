use super::prelude::*;

pub type ApiVersionsRequest = ApiVersionsRequest3;
pub type ApiVersionsResponse = ApiVersionsResponse3;
impl ApiCall for ApiVersionsRequest {
    type Response = ApiVersionsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ApiVersions
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => true,
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
                ApiVersionsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                ApiVersionsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &ApiVersionsRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &ApiVersionsRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &ApiVersionsRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, ApiVersionsResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => ApiVersionsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => ApiVersionsResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => ApiVersionsResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => ApiVersionsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => ApiVersionsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ApiVersionsRequest0 {}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ApiVersionsRequest1 {}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ApiVersionsRequest2 {}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ApiVersionsRequest3 {
    pub client_software_name: Optional<String>,
    pub client_software_version: Optional<String>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponse0 {
    pub error_code: Int16,
    pub api_keys: Vec<ApiVersionsResponseApiKeys0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponseApiKeys0 {
    pub api_key: Int16,
    pub min_version: Int16,
    pub max_version: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponse1 {
    pub error_code: Int16,
    pub api_keys: Vec<ApiVersionsResponseApiKeys1>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponseApiKeys1 {
    pub api_key: Int16,
    pub min_version: Int16,
    pub max_version: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponse2 {
    pub error_code: Int16,
    pub api_keys: Vec<ApiVersionsResponseApiKeys2>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponseApiKeys2 {
    pub api_key: Int16,
    pub min_version: Int16,
    pub max_version: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponse3 {
    pub error_code: Int16,
    pub api_keys: Vec<ApiVersionsResponseApiKeys3>,
    pub throttle_time_ms: Optional<Int32>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ApiVersionsResponseApiKeys3 {
    pub api_key: Int16,
    pub min_version: Int16,
    pub max_version: Int16,
    pub tag_buffer: Optional<TagBuffer>,
}

impl TryFrom<ApiVersionsRequest3> for ApiVersionsRequest0 {
    type Error = Error;
    fn try_from(latest: ApiVersionsRequest3) -> Result<Self, Self::Error> {
        Ok(ApiVersionsRequest0 {})
    }
}

impl TryFrom<ApiVersionsRequest3> for ApiVersionsRequest1 {
    type Error = Error;
    fn try_from(latest: ApiVersionsRequest3) -> Result<Self, Self::Error> {
        Ok(ApiVersionsRequest1 {})
    }
}

impl TryFrom<ApiVersionsRequest3> for ApiVersionsRequest2 {
    type Error = Error;
    fn try_from(latest: ApiVersionsRequest3) -> Result<Self, Self::Error> {
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
            ..ApiVersionsResponseApiKeys3::default()
        }
    }
}

impl From<ApiVersionsResponse1> for ApiVersionsResponse3 {
    fn from(older: ApiVersionsResponse1) -> Self {
        ApiVersionsResponse3 {
            error_code: older.error_code,
            api_keys: older.api_keys.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
            ..ApiVersionsResponse3::default()
        }
    }
}

impl From<ApiVersionsResponseApiKeys1> for ApiVersionsResponseApiKeys3 {
    fn from(older: ApiVersionsResponseApiKeys1) -> Self {
        ApiVersionsResponseApiKeys3 {
            api_key: older.api_key,
            min_version: older.min_version,
            max_version: older.max_version,
            ..ApiVersionsResponseApiKeys3::default()
        }
    }
}

impl From<ApiVersionsResponse2> for ApiVersionsResponse3 {
    fn from(older: ApiVersionsResponse2) -> Self {
        ApiVersionsResponse3 {
            error_code: older.error_code,
            api_keys: older.api_keys.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
            ..ApiVersionsResponse3::default()
        }
    }
}

impl From<ApiVersionsResponseApiKeys2> for ApiVersionsResponseApiKeys3 {
    fn from(older: ApiVersionsResponseApiKeys2) -> Self {
        ApiVersionsResponseApiKeys3 {
            api_key: older.api_key,
            min_version: older.min_version,
            max_version: older.max_version,
            ..ApiVersionsResponseApiKeys3::default()
        }
    }
}
