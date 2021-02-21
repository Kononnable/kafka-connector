use super::prelude::*;
pub type DescribeConfigsRequest = DescribeConfigsRequest0;
impl ApiCall for DescribeConfigsRequest0 {
    type Response = DescribeConfigsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeConfigs
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        false
    }
    fn serialize(&self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(self, buf, Self::is_flexible_version(version), version);
    }
    fn deserialize_response(version: u16, buf: &mut Bytes) -> (i32, Self::Response) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse::deserialize(buf, false, 2).correlation,
            false => HeaderResponse::deserialize(buf, false, 1).correlation,
        };
        let response =
            Self::Response::deserialize(buf, Self::is_flexible_version(version), version);
        (correlation, response)
    }
    fn deserialize_request(version: u16, buf: &mut Bytes) -> (OwnedHeaderRequest, Self) {
        let header = match Self::is_flexible_version(version) {
            true => OwnedHeaderRequest::deserialize(buf, false, 2),
            false => OwnedHeaderRequest::deserialize(buf, false, 1),
        };
        let request = Self::deserialize(buf, Self::is_flexible_version(version), version);
        (header, request)
    }
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct DescribeConfigsRequest0 {
    #[min_version = 0]
    pub resources: Vec<DescribeConfigsRequestResources0>,
    #[min_version = 1]
    pub include_synonyms: Boolean,
    #[min_version = 3]
    pub include_documentation: Boolean,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct DescribeConfigsRequestResources0 {
    #[min_version = 0]
    pub resource_type: Int8,
    #[min_version = 0]
    pub resource_name: String,
    #[min_version = 0]
    pub configuration_keys: Vec<String>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub results: Vec<DescribeConfigsResponseResults0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResults0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub resource_type: Int8,
    #[min_version = 0]
    pub resource_name: String,
    #[min_version = 0]
    pub configs: Vec<DescribeConfigsResponseResultsConfigs0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigs0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub value: NullableString,
    #[min_version = 0]
    pub read_only: Boolean,
    #[min_version = 1]
    pub config_source: Option<Int8>,
    #[min_version = 0]
    pub is_sensitive: Boolean,
    #[min_version = 1]
    pub synonyms: Option<Vec<DescribeConfigsResponseResultsConfigsSynonyms0>>,
    #[min_version = 3]
    pub config_type: Option<Int8>,
    #[min_version = 3]
    pub documentation: Option<NullableString>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigsSynonyms0 {
    #[min_version = 1]
    pub name: Option<String>,
    #[min_version = 1]
    pub value: Option<NullableString>,
    #[min_version = 1]
    pub source: Option<Int8>,
}

impl DescribeConfigsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DescribeConfigsResponseResults0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.configs.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DescribeConfigsResponseResultsConfigs0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if let Some(vec) = self.synonyms.as_ref() {
            for item in vec {
                if let Some(x) = item.get_first_error() {
                    return Some(x);
                };
            }
        }
        None
    }
}
impl DescribeConfigsResponseResultsConfigsSynonyms0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
