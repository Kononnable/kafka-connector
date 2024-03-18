use super::super::prelude::*;

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
}

impl AlterConfigsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.resources != IndexMap::<AlterConfigsResourceKey, AlterConfigsResource>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "resources",
                _version,
                "AlterConfigsRequest",
            ));
        }
        if self.validate_only != bool::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "validate_only",
                _version,
                "AlterConfigsRequest",
            ));
        }
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
        if self.resource_type != i8::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "resource_type",
                _version,
                "AlterConfigsResourceKey",
            ));
        }
        if self.resource_name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "resource_name",
                _version,
                "AlterConfigsResourceKey",
            ));
        }
        Ok(())
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
        if self.configs != IndexMap::<AlterableConfigKey, AlterableConfig>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "configs",
                _version,
                "AlterConfigsResource",
            ));
        }
        Ok(())
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
        if self.name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "AlterableConfigKey",
            ));
        }
        Ok(())
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
        if self.value.is_some() && self.value != Some(String::default()) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "value",
                _version,
                "AlterableConfig",
            ));
        }
        Ok(())
    }
}
