use super::super::prelude::*;

/// Version 1 adds IncludeSynoyms.
/// Version 2 is the same as version 1.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeConfigsRequest {
    /// The resources whose configurations we want to describe.
    pub resources: Vec<DescribeConfigsResource>,

    /// True if we should include all synonyms.
    pub include_synoyms: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribeConfigsResource {
    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,

    /// The configuration keys to list, or null to list all configuration keys.
    pub configuration_keys: Option<Vec<String>>,
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
        if version >= 1 {
            self.include_synoyms.serialize(version, bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let resources = Vec::<DescribeConfigsResource>::deserialize(version, bytes);
        let include_synoyms = if version >= 1 {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribeConfigsRequest {
            resources,
            include_synoyms,
        }
    }
}

impl DescribeConfigsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.include_synoyms != bool::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "include_synoyms",
                _version,
                "DescribeConfigsRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for DescribeConfigsResource {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.resource_type.serialize(version, bytes)?;
        self.resource_name.serialize(version, bytes)?;
        self.configuration_keys.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribeConfigsResource {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.configuration_keys.is_none() {
            return Err(SerializationError::NullValue(
                "configuration_keys",
                _version,
                "DescribeConfigsResource",
            ));
        }
        Ok(())
    }
}

impl FromBytes for DescribeConfigsResource {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let resource_type = i8::deserialize(version, bytes);
        let resource_name = String::deserialize(version, bytes);
        let configuration_keys = Option::<Vec<String>>::deserialize(version, bytes);
        DescribeConfigsResource {
            resource_type,
            resource_name,
            configuration_keys,
        }
    }
}
