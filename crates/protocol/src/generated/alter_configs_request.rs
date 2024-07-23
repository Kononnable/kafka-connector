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

    fn get_api_key() -> i16 {
        33
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.resources.serialize(version, bytes)?;
        self.validate_only.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
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
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for AlterConfigsResourceKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.resource_type.serialize(version, bytes)?;
        self.resource_name.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterConfigsResourceKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterConfigsResourceKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let resource_type = i8::deserialize(version, bytes);
        let resource_name = String::deserialize(version, bytes);
        AlterConfigsResourceKey {
            resource_type,
            resource_name,
        }
    }
}

impl ToBytes for AlterConfigsResource {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.configs.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterConfigsResource {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterConfigsResource {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let configs = IndexMap::<AlterableConfigKey, AlterableConfig>::deserialize(version, bytes);
        AlterConfigsResource { configs }
    }
}

impl ToBytes for AlterableConfigKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterableConfigKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterableConfigKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        AlterableConfigKey { name }
    }
}

impl ToBytes for AlterableConfig {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.value.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterableConfig {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.value.is_none() {
            return Err(SerializationError::NullValue(
                "value",
                _version,
                "AlterableConfig",
            ));
        }
        Ok(())
    }
}

impl FromBytes for AlterableConfig {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let value = Option::<String>::deserialize(version, bytes);
        AlterableConfig { value }
    }
}
