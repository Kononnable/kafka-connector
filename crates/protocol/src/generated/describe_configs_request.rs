use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeConfigsRequest {
    pub resources: Vec<DescribeConfigsResource>,
    pub include_synoyms: bool,
}

#[derive(Debug, Default, Clone)]
pub struct DescribeConfigsResource {
    pub resource_type: i8,
    pub resource_name: String,
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
