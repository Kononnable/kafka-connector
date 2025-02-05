use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreatePartitionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The partition creation results for each topic.
    pub results: Vec<CreatePartitionsTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatePartitionsTopicResult {
    /// The topic name.
    pub name: String,

    /// The result error, or zero if there was no error.
    pub error_code: i16,

    /// The result message, or null if there was no error.
    pub error_message: Option<String>,
}

impl ApiResponse for CreatePartitionsResponse {
    type Request = super::create_partitions_request::CreatePartitionsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(37)
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
        self.throttle_time_ms.serialize(version, _bytes)?;
        self.results.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<CreatePartitionsTopicResult>::deserialize(version, bytes);
        CreatePartitionsResponse {
            throttle_time_ms,
            results,
        }
    }
}

impl CreatePartitionsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatePartitionsTopicResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        self.error_code.serialize(version, _bytes)?;
        self.error_message.serialize(version, _bytes)?;
        Ok(())
    }
}

impl CreatePartitionsTopicResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatePartitionsTopicResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        CreatePartitionsTopicResult {
            name,
            error_code,
            error_message,
        }
    }
}
