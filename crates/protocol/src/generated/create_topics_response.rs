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

    fn get_api_key() -> i16 {
        19
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        3
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
        if version >= 2 {
            self.throttle_time_ms.serialize(version, bytes)?;
        }
        self.topics.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 2 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics =
            IndexMap::<CreatableTopicResultKey, CreatableTopicResult>::deserialize(version, bytes);
        (
            header,
            CreateTopicsResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl CreateTopicsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatableTopicResultKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreatableTopicResultKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatableTopicResultKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        CreatableTopicResultKey { name }
    }
}

impl ToBytes for CreatableTopicResult {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, bytes)?;
        if version >= 1 {
            self.error_message.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl CreatableTopicResult {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.error_message.is_none() && !_version >= 1 {
            return Err(SerializationError::NullValue(
                "error_message",
                _version,
                "CreatableTopicResult",
            ));
        }
        Ok(())
    }
}

impl FromBytes for CreatableTopicResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = if version >= 1 {
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
