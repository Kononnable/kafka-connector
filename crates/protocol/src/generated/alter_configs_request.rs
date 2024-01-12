use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AlterConfigsRequest {
    /// The updates for each resource.
    pub resources: Vec<AlterConfigsResource>,

    /// True if we should validate the request, but not change the configurations.
    pub validate_only: bool,
}

#[derive(Clone, Debug, Default)]
pub struct AlterConfigsResource {
    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,

    /// The configurations.
    pub configs: Vec<AlterableConfig>,
}

#[derive(Clone, Debug, Default)]
pub struct AlterableConfig {
    /// The configuration key name.
    pub name: String,

    /// The value to set for the configuration key.
    pub value: String,
}

impl ApiRequest for AlterConfigsRequest {
    type Response = super::alter_configs_response::AlterConfigsResponse;

    fn get_api_key() -> i16 {
        33
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
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
        if version >= 0 {
            self.validate_only.serialize(version, bytes);
        }
    }
}

impl ToBytes for AlterConfigsResource {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.resource_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.resource_name.serialize(version, bytes);
        }
        if version >= 0 {
            self.configs.serialize(version, bytes);
        }
    }
}

impl ToBytes for AlterableConfig {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.value.serialize(version, bytes);
        }
    }
}
