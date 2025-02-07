use super::super::prelude::*;

/// Version 1 adds ConfigSource and the synonyms.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
/// Note: the v0 default for this field that shouldd be exposed to callers is
/// context-dependent. For example, if the resource is a broker, this should default to 4.
/// -1 is just a placeholder value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeConfigsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each resource.
    pub results: Vec<DescribeConfigsResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribeConfigsResult {
    /// The error code, or 0 if we were able to successfully describe the configurations.
    pub error_code: i16,

    /// The error message, or null if we were able to successfully describe the configurations.
    pub error_message: Option<String>,

    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,

    /// Each listed configuration.
    pub configs: Vec<DescribeConfigsResourceResult>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeConfigsResourceResult {
    /// The configuration name.
    pub name: String,

    /// The configuration value.
    pub value: Option<String>,

    /// True if the configuration is read-only.
    pub read_only: bool,

    /// True if the configuration is not set.
    pub is_default: bool,

    /// The configuration source.
    pub config_source: i8,

    /// True if this configuration is sensitive.
    pub is_sensitive: bool,

    /// The synonyms for this configuration key.
    pub synonyms: Vec<DescribeConfigsSynonym>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribeConfigsSynonym {
    /// The synonym name.
    pub name: String,

    /// The synonym value.
    pub value: Option<String>,

    /// The synonym source.
    pub source: i8,
}

impl ApiResponse for DescribeConfigsResponse {
    type Request = super::describe_configs_request::DescribeConfigsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(32)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(2)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.throttle_time_ms.serialize(version, _bytes);
        self.results.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<DescribeConfigsResult>::deserialize(version, bytes);
        DescribeConfigsResponse {
            throttle_time_ms,
            results,
        }
    }
}

impl DescribeConfigsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.results.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for DescribeConfigsResult {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.error_code.serialize(version, _bytes);
        self.error_message.serialize(version, _bytes);
        self.resource_type.serialize(version, _bytes);
        self.resource_name.serialize(version, _bytes);
        self.configs.serialize(version, _bytes);
    }
}

impl DescribeConfigsResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.configs.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for DescribeConfigsResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let resource_type = i8::deserialize(version, bytes);
        let resource_name = String::deserialize(version, bytes);
        let configs = Vec::<DescribeConfigsResourceResult>::deserialize(version, bytes);
        DescribeConfigsResult {
            error_code,
            error_message,
            resource_type,
            resource_name,
            configs,
        }
    }
}

impl ToBytes for DescribeConfigsResourceResult {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.value.serialize(version, _bytes);
        self.read_only.serialize(version, _bytes);
        if version >= ApiVersion(0) {
            self.is_default.serialize(version, _bytes);
        }
        if version >= ApiVersion(1) {
            self.config_source.serialize(version, _bytes);
        }
        self.is_sensitive.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.synonyms.serialize(version, _bytes);
        }
    }
}

impl DescribeConfigsResourceResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.synonyms.iter() {
            item.validate_fields(_version)?;
        }
        if self.is_default != bool::default() && _version.0 < 0 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "is_default",
                *_version,
                "DescribeConfigsResourceResult",
            ));
        }
        Ok(())
    }
}

impl FromBytes for DescribeConfigsResourceResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let value = Option::<String>::deserialize(version, bytes);
        let read_only = bool::deserialize(version, bytes);
        let is_default = if version >= ApiVersion(0) {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let config_source = if version >= ApiVersion(1) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let is_sensitive = bool::deserialize(version, bytes);
        let synonyms = if version >= ApiVersion(1) {
            Vec::<DescribeConfigsSynonym>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribeConfigsResourceResult {
            name,
            value,
            read_only,
            is_default,
            config_source,
            is_sensitive,
            synonyms,
        }
    }
}

impl Default for DescribeConfigsResourceResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
            read_only: Default::default(),
            is_default: Default::default(),
            config_source: -1,
            is_sensitive: Default::default(),
            synonyms: Default::default(),
        }
    }
}

impl ToBytes for DescribeConfigsSynonym {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        if version >= ApiVersion(1) {
            self.name.serialize(version, _bytes);
        }
        if version >= ApiVersion(1) {
            self.value.serialize(version, _bytes);
        }
        if version >= ApiVersion(1) {
            self.source.serialize(version, _bytes);
        }
    }
}

impl DescribeConfigsSynonym {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.name != String::default() && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                *_version,
                "DescribeConfigsSynonym",
            ));
        }
        if self.value != Some(String::default()) && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "value",
                *_version,
                "DescribeConfigsSynonym",
            ));
        }
        if self.source != i8::default() && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "source",
                *_version,
                "DescribeConfigsSynonym",
            ));
        }
        Ok(())
    }
}

impl FromBytes for DescribeConfigsSynonym {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = if version >= ApiVersion(1) {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let value = if version >= ApiVersion(1) {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let source = if version >= ApiVersion(1) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribeConfigsSynonym {
            name,
            value,
            source,
        }
    }
}
