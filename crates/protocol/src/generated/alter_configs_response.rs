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
        header: &ResponseHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.throttle_time_ms.serialize(version, bytes)?;
        self.resources.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let resources = Vec::<AlterConfigsResourceResponse>::deserialize(version, bytes);
        (
            header,
            AlterConfigsResponse {
                throttle_time_ms,
                resources,
            },
        )
    }
}

impl AlterConfigsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for AlterConfigsResourceResponse {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, bytes)?;
        self.error_message.serialize(version, bytes)?;
        self.resource_type.serialize(version, bytes)?;
        self.resource_name.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterConfigsResourceResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.error_message.is_none() {
            return Err(SerializationError::NullValue(
                "error_message",
                _version,
                "AlterConfigsResourceResponse",
            ));
        }
        Ok(())
    }
}

impl FromBytes for AlterConfigsResourceResponse {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
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
