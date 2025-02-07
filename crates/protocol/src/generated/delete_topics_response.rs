use super::super::prelude::*;

/// Version 1 adds the throttle time.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
/// Starting in version 3, a TOPIC_DELETION_DISABLED error code may be returned.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub responses: Vec<DeletableTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeletableTopicResult {
    /// The topic name
    pub name: String,

    /// The deletion error, or 0 if the deletion succeeded.
    pub error_code: i16,
}

impl ApiResponse for DeleteTopicsResponse {
    type Request = super::delete_topics_request::DeleteTopicsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(20)
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
        if version >= ApiVersion(1) {
            self.throttle_time_ms.serialize(version, _bytes);
        }
        self.responses.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let responses = Vec::<DeletableTopicResult>::deserialize(version, bytes);
        DeleteTopicsResponse {
            throttle_time_ms,
            responses,
        }
    }
}

impl DeleteTopicsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.responses.iter() {
            item.validate_fields(_version)?;
        }
        if self.throttle_time_ms != i32::default() && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "throttle_time_ms",
                *_version,
                "DeleteTopicsResponse",
            ));
        }
        Ok(())
    }
}

impl ToBytes for DeletableTopicResult {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.error_code.serialize(version, _bytes);
    }
}

impl DeletableTopicResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeletableTopicResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        DeletableTopicResult { name, error_code }
    }
}
