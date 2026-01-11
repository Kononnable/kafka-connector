use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlterConfigsRequest {
    /// The updates for each resource.
    pub resources: IndexMap<AlterConfigsResourceKey, AlterConfigsResource>,

    /// True if we should validate the request, but not change the configurations.
    pub validate_only: bool,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AlterConfigsResourceKey {
    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterConfigsResource {
    /// The configurations.
    pub configs: IndexMap<AlterableConfigKey, AlterableConfig>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AlterableConfigKey {
    /// The configuration key name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterableConfig {
    /// The value to set for the configuration key.
    pub value: Option<String>,
}

impl ApiRequest for AlterConfigsRequest {
    type Response = super::alter_configs_response::AlterConfigsResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(33)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(1)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.resources.serialize(version, _bytes);
        self.validate_only.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let resources =
            IndexMap::<AlterConfigsResourceKey, AlterConfigsResource>::deserialize(version, bytes);
        let validate_only = bool::deserialize(version, bytes);
        AlterConfigsRequest {
            resources,
            validate_only,
        }
    }
}

impl AlterConfigsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.resources.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for IndexMap<AlterConfigsResourceKey, AlterConfigsResource> {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        _bytes.put_i32(self.len() as i32);
        for (key, value) in self {
            key.resource_type.serialize(version, _bytes);
            key.resource_name.serialize(version, _bytes);
            value.configs.serialize(version, _bytes);
        }
    }
}

impl AlterConfigsResourceKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl AlterConfigsResource {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.configs.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for IndexMap<AlterConfigsResourceKey, AlterConfigsResource> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        let mut ret = IndexMap::with_capacity(cap as usize);
        for _ in 0..cap {
            let resource_type = i8::deserialize(version, bytes);
            let resource_name = String::deserialize(version, bytes);
            let configs =
                IndexMap::<AlterableConfigKey, AlterableConfig>::deserialize(version, bytes);
            let key = AlterConfigsResourceKey {
                resource_type,
                resource_name,
            };
            let value = AlterConfigsResource { configs };
            ret.insert(key, value);
        }

        ret
    }
}

impl ToBytes for IndexMap<AlterableConfigKey, AlterableConfig> {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        _bytes.put_i32(self.len() as i32);
        for (key, value) in self {
            key.name.serialize(version, _bytes);
            value.value.serialize(version, _bytes);
        }
    }
}

impl AlterableConfigKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl AlterableConfig {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for IndexMap<AlterableConfigKey, AlterableConfig> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        let mut ret = IndexMap::with_capacity(cap as usize);
        for _ in 0..cap {
            let name = String::deserialize(version, bytes);
            let value = Option::<String>::deserialize(version, bytes);
            let key = AlterableConfigKey { name };
            let value = AlterableConfig { value };
            ret.insert(key, value);
        }

        ret
    }
}
