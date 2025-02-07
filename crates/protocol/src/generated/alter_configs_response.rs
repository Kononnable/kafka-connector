use super::super::prelude::*;

/// Starting in version 1, on quota violation brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlterConfigsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses for each resource.
    pub resources: Vec<AlterConfigsResourceResponse>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterConfigsResourceResponse {
    /// The resource error code.
    pub error_code: i16,

    /// The resource error message, or null if there was no error.
    pub error_message: Option<String>,

    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,
}

impl ApiResponse for AlterConfigsResponse {
    type Request = super::alter_configs_request::AlterConfigsRequest;

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
        self.throttle_time_ms.serialize(version, _bytes);
        self.resources.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let resources = Vec::<AlterConfigsResourceResponse>::deserialize(version, bytes);
        AlterConfigsResponse {
            throttle_time_ms,
            resources,
        }
    }
}

impl AlterConfigsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.resources.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for AlterConfigsResourceResponse {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.error_code.serialize(version, _bytes);
        self.error_message.serialize(version, _bytes);
        self.resource_type.serialize(version, _bytes);
        self.resource_name.serialize(version, _bytes);
    }
}

impl AlterConfigsResourceResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterConfigsResourceResponse {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let resource_type = i8::deserialize(version, bytes);
        let resource_name = String::deserialize(version, bytes);
        AlterConfigsResourceResponse {
            error_code,
            error_message,
            resource_type,
            resource_name,
        }
    }
}
