use super::prelude::*;

pub type DescribeConfigsRequest = DescribeConfigsRequest3;
pub type DescribeConfigsResponse = DescribeConfigsResponse3;
impl ApiCall for DescribeConfigsRequest {
    type Response = DescribeConfigsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeConfigs
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
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
                DescribeConfigsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DescribeConfigsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &DescribeConfigsRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &DescribeConfigsRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &DescribeConfigsRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DescribeConfigsResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => DescribeConfigsResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => DescribeConfigsResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => DescribeConfigsResponse2::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            3 => DescribeConfigsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => DescribeConfigsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeConfigsRequest0 {
    pub resources: Vec<DescribeConfigsRequestResources0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeConfigsRequestResources0 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configuration_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeConfigsRequest1 {
    pub resources: Vec<DescribeConfigsRequestResources1>,
    pub include_synonyms: Optional<Boolean>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeConfigsRequestResources1 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configuration_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeConfigsRequest2 {
    pub resources: Vec<DescribeConfigsRequestResources2>,
    pub include_synonyms: Optional<Boolean>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeConfigsRequestResources2 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configuration_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeConfigsRequest3 {
    pub resources: Vec<DescribeConfigsRequestResources3>,
    pub include_synonyms: Optional<Boolean>,
    pub include_documentation: Optional<Boolean>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeConfigsRequestResources3 {
    pub resource_type: Int8,
    pub resource_name: String,
    pub configuration_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DescribeConfigsResponseResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResults0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<DescribeConfigsResponseResultsConfigs0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigs0 {
    pub name: String,
    pub value: NullableString,
    pub read_only: Boolean,
    pub is_default: Boolean,
    pub is_sensitive: Boolean,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DescribeConfigsResponseResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResults1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<DescribeConfigsResponseResultsConfigs1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigs1 {
    pub name: String,
    pub value: NullableString,
    pub read_only: Boolean,
    pub config_source: Optional<Int8>,
    pub is_sensitive: Boolean,
    pub synonyms: Optional<Vec<DescribeConfigsResponseResultsConfigsSynonyms1>>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigsSynonyms1 {
    pub name: String,
    pub value: NullableString,
    pub source: Int8,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DescribeConfigsResponseResults2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResults2 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<DescribeConfigsResponseResultsConfigs2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigs2 {
    pub name: String,
    pub value: NullableString,
    pub read_only: Boolean,
    pub config_source: Optional<Int8>,
    pub is_sensitive: Boolean,
    pub synonyms: Optional<Vec<DescribeConfigsResponseResultsConfigsSynonyms2>>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigsSynonyms2 {
    pub name: String,
    pub value: NullableString,
    pub source: Int8,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponse3 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DescribeConfigsResponseResults3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResults3 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub resource_type: Int8,
    pub resource_name: String,
    pub configs: Vec<DescribeConfigsResponseResultsConfigs3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigs3 {
    pub name: String,
    pub value: NullableString,
    pub read_only: Boolean,
    pub config_source: Optional<Int8>,
    pub is_sensitive: Boolean,
    pub synonyms: Optional<Vec<DescribeConfigsResponseResultsConfigsSynonyms3>>,
    pub config_type: Optional<Int8>,
    pub documentation: Optional<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeConfigsResponseResultsConfigsSynonyms3 {
    pub name: String,
    pub value: NullableString,
    pub source: Int8,
}

impl TryFrom<DescribeConfigsRequest3> for DescribeConfigsRequest0 {
    type Error = Error;
    fn try_from(latest: DescribeConfigsRequest3) -> Result<Self, Self::Error> {
        if latest.include_synonyms.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeConfigsRequest",
                0,
                "include_synonyms",
            ));
        }
        if latest.include_documentation.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeConfigsRequest",
                0,
                "include_documentation",
            ));
        }
        Ok(DescribeConfigsRequest0 {
            resources: latest
                .resources
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DescribeConfigsRequestResources3> for DescribeConfigsRequestResources0 {
    type Error = Error;
    fn try_from(latest: DescribeConfigsRequestResources3) -> Result<Self, Self::Error> {
        Ok(DescribeConfigsRequestResources0 {
            resource_type: latest.resource_type,
            resource_name: latest.resource_name,
            configuration_keys: latest.configuration_keys,
        })
    }
}

impl TryFrom<DescribeConfigsRequest3> for DescribeConfigsRequest1 {
    type Error = Error;
    fn try_from(latest: DescribeConfigsRequest3) -> Result<Self, Self::Error> {
        if latest.include_documentation.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeConfigsRequest",
                1,
                "include_documentation",
            ));
        }
        Ok(DescribeConfigsRequest1 {
            resources: latest
                .resources
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            include_synonyms: latest.include_synonyms,
        })
    }
}

impl TryFrom<DescribeConfigsRequestResources3> for DescribeConfigsRequestResources1 {
    type Error = Error;
    fn try_from(latest: DescribeConfigsRequestResources3) -> Result<Self, Self::Error> {
        Ok(DescribeConfigsRequestResources1 {
            resource_type: latest.resource_type,
            resource_name: latest.resource_name,
            configuration_keys: latest.configuration_keys,
        })
    }
}

impl TryFrom<DescribeConfigsRequest3> for DescribeConfigsRequest2 {
    type Error = Error;
    fn try_from(latest: DescribeConfigsRequest3) -> Result<Self, Self::Error> {
        if latest.include_documentation.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeConfigsRequest",
                2,
                "include_documentation",
            ));
        }
        Ok(DescribeConfigsRequest2 {
            resources: latest
                .resources
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            include_synonyms: latest.include_synonyms,
        })
    }
}

impl TryFrom<DescribeConfigsRequestResources3> for DescribeConfigsRequestResources2 {
    type Error = Error;
    fn try_from(latest: DescribeConfigsRequestResources3) -> Result<Self, Self::Error> {
        Ok(DescribeConfigsRequestResources2 {
            resource_type: latest.resource_type,
            resource_name: latest.resource_name,
            configuration_keys: latest.configuration_keys,
        })
    }
}

impl From<DescribeConfigsResponse0> for DescribeConfigsResponse3 {
    fn from(older: DescribeConfigsResponse0) -> Self {
        DescribeConfigsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeConfigsResponseResults0> for DescribeConfigsResponseResults3 {
    fn from(older: DescribeConfigsResponseResults0) -> Self {
        DescribeConfigsResponseResults3 {
            error_code: older.error_code,
            error_message: older.error_message,
            resource_type: older.resource_type,
            resource_name: older.resource_name,
            configs: older.configs.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeConfigsResponseResultsConfigs0> for DescribeConfigsResponseResultsConfigs3 {
    fn from(older: DescribeConfigsResponseResultsConfigs0) -> Self {
        DescribeConfigsResponseResultsConfigs3 {
            name: older.name,
            value: older.value,
            read_only: older.read_only,
            is_sensitive: older.is_sensitive,
            ..DescribeConfigsResponseResultsConfigs3::default()
        }
    }
}

impl From<DescribeConfigsResponse1> for DescribeConfigsResponse3 {
    fn from(older: DescribeConfigsResponse1) -> Self {
        DescribeConfigsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeConfigsResponseResults1> for DescribeConfigsResponseResults3 {
    fn from(older: DescribeConfigsResponseResults1) -> Self {
        DescribeConfigsResponseResults3 {
            error_code: older.error_code,
            error_message: older.error_message,
            resource_type: older.resource_type,
            resource_name: older.resource_name,
            configs: older.configs.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeConfigsResponseResultsConfigs1> for DescribeConfigsResponseResultsConfigs3 {
    fn from(older: DescribeConfigsResponseResultsConfigs1) -> Self {
        DescribeConfigsResponseResultsConfigs3 {
            name: older.name,
            value: older.value,
            read_only: older.read_only,
            config_source: older.config_source,
            is_sensitive: older.is_sensitive,
            synonyms: older
                .synonyms
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            ..DescribeConfigsResponseResultsConfigs3::default()
        }
    }
}

impl From<DescribeConfigsResponseResultsConfigsSynonyms1>
    for DescribeConfigsResponseResultsConfigsSynonyms3
{
    fn from(older: DescribeConfigsResponseResultsConfigsSynonyms1) -> Self {
        DescribeConfigsResponseResultsConfigsSynonyms3 {
            name: older.name,
            value: older.value,
            source: older.source,
        }
    }
}

impl From<DescribeConfigsResponse2> for DescribeConfigsResponse3 {
    fn from(older: DescribeConfigsResponse2) -> Self {
        DescribeConfigsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeConfigsResponseResults2> for DescribeConfigsResponseResults3 {
    fn from(older: DescribeConfigsResponseResults2) -> Self {
        DescribeConfigsResponseResults3 {
            error_code: older.error_code,
            error_message: older.error_message,
            resource_type: older.resource_type,
            resource_name: older.resource_name,
            configs: older.configs.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeConfigsResponseResultsConfigs2> for DescribeConfigsResponseResultsConfigs3 {
    fn from(older: DescribeConfigsResponseResultsConfigs2) -> Self {
        DescribeConfigsResponseResultsConfigs3 {
            name: older.name,
            value: older.value,
            read_only: older.read_only,
            config_source: older.config_source,
            is_sensitive: older.is_sensitive,
            synonyms: older
                .synonyms
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
            ..DescribeConfigsResponseResultsConfigs3::default()
        }
    }
}

impl From<DescribeConfigsResponseResultsConfigsSynonyms2>
    for DescribeConfigsResponseResultsConfigsSynonyms3
{
    fn from(older: DescribeConfigsResponseResultsConfigsSynonyms2) -> Self {
        DescribeConfigsResponseResultsConfigsSynonyms3 {
            name: older.name,
            value: older.value,
            source: older.source,
        }
    }
}
