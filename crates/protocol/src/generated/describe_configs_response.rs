use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeConfigsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each resource.
    pub results: Vec<DescribeConfigsResult>,
}

#[derive(Clone, Debug, Default)]
pub struct DescribeConfigsResult {
    /// The error code, or 0 if we were able to successfully describe the configurations.
    pub error_code: i16,

    /// The error message, or null if we were able to successfully describe the configurations.
    pub error_message: String,

    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,

    /// Each listed configuration.
    pub configs: Vec<DescribeConfigsResourceResult>,
}

#[derive(Clone, Debug)]
pub struct DescribeConfigsResourceResult {
    /// The configuration name.
    pub name: String,

    /// The configuration value.
    pub value: String,

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

#[derive(Clone, Debug, Default)]
pub struct DescribeConfigsSynonym {
    /// The synonym name.
    pub name: String,

    /// The synonym value.
    pub value: String,

    /// The synonym source.
    pub source: i8,
}

impl ApiResponse for DescribeConfigsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<DescribeConfigsResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            DescribeConfigsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for DescribeConfigsResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_message = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resource_type = if version >= 0 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resource_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let configs = if version >= 0 {
            Vec::<DescribeConfigsResourceResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribeConfigsResult {
            error_code,
            error_message,
            resource_type,
            resource_name,
            configs,
        }
    }
}

impl FromBytes for DescribeConfigsResourceResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let value = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let read_only = if version >= 0 {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let is_default = if version >= 0 {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let config_source = if version >= 1 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let is_sensitive = if version >= 0 {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let synonyms = if version >= 1 {
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

impl FromBytes for DescribeConfigsSynonym {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 1 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let value = if version >= 1 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let source = if version >= 1 {
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
