use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeConfigsResponse {
    pub throttle_time_ms: i32,
    pub results: Vec<DescribeConfigsResult>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribeConfigsResult {
    pub error_code: i16,
    pub error_message: String,
    pub resource_type: i8,
    pub resource_name: String,
    pub configs: Vec<DescribeConfigsResourceResult>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribeConfigsResourceResult {
    pub name: String,
    pub value: String,
    pub read_only: bool,
    pub is_default: bool,
    pub config_source: i8,
    pub is_sensitive: bool,
    pub synonyms: Vec<DescribeConfigsSynonym>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribeConfigsSynonym {
    pub name: String,
    pub value: String,
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
