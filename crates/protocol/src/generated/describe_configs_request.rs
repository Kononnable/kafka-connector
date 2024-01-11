use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DescribeConfigsRequest {
    /// The resources whose configurations we want to describe.
    pub resources: Vec<DescribeConfigsResource>,

    /// True if we should include all synonyms.
    pub include_synoyms: bool,
}

#[derive(Debug, Clone)]
pub struct DescribeConfigsResource {
    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,

    /// The configuration keys to list, or null to list all configuration keys.
    pub configuration_keys: Vec<String>,
}

impl ApiRequest for DescribeConfigsRequest {
    type Response = super::describe_configs_response::DescribeConfigsResponse;

    fn get_api_key() -> i16 {
        32
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
    }

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.resources.serialize(version, bytes);
        }
        if version >= 1 {
            self.include_synoyms.serialize(version, bytes);
        }
    }
}

impl Default for DescribeConfigsRequest {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            include_synoyms: false,
        }
    }
}

impl ToBytes for DescribeConfigsResource {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.resource_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.resource_name.serialize(version, bytes);
        }
        if version >= 0 {
            self.configuration_keys.serialize(version, bytes);
        }
    }
}

impl Default for DescribeConfigsResource {
    fn default() -> Self {
        Self {
            resource_type: Default::default(),
            resource_name: Default::default(),
            configuration_keys: Default::default(),
        }
    }
}
