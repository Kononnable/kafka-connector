use super::super::prelude::*;

/// Version 1 adds a per-topic error message string.
/// Version 2 adds the throttle time.
/// Starting in version 3, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreateTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Results for each topic we tried to create.
    pub topics: IndexMap<CreatableTopicResultKey, CreatableTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct CreatableTopicResultKey {
    /// The topic name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatableTopicResult {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The error message, or null if there was no error.
    pub error_message: Option<String>,
}

impl ApiResponse for CreateTopicsResponse {
    type Request = super::create_topics_request::CreateTopicsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(19)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(3)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        if version >= ApiVersion(2) {
            self.throttle_time_ms.serialize(version, _bytes)?;
        }
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(2) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics =
            IndexMap::<CreatableTopicResultKey, CreatableTopicResult>::deserialize(version, bytes);
        CreateTopicsResponse {
            throttle_time_ms,
            topics,
        }
    }
}

impl CreateTopicsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatableTopicResultKey {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        Ok(())
    }
}

impl CreatableTopicResultKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatableTopicResultKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        CreatableTopicResultKey { name }
    }
}

impl ToBytes for CreatableTopicResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.error_message.serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl CreatableTopicResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatableTopicResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = if version >= ApiVersion(1) {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        CreatableTopicResult {
            error_code,
            error_message,
        }
    }
}
