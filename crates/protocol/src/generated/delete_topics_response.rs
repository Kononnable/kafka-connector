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

    fn get_api_key() -> i16 {
        20
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
        if version >= 1 {
            self.throttle_time_ms.serialize(version, bytes)?;
        }
        self.responses.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let responses = Vec::<DeletableTopicResult>::deserialize(version, bytes);
        (
            header,
            DeleteTopicsResponse {
                throttle_time_ms,
                responses,
            },
        )
    }
}

impl DeleteTopicsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.throttle_time_ms != i32::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "throttle_time_ms",
                _version,
                "DeleteTopicsResponse",
            ));
        }
        Ok(())
    }
}

impl ToBytes for DeletableTopicResult {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        Ok(())
    }
}

impl DeletableTopicResult {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeletableTopicResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        DeletableTopicResult { name, error_code }
    }
}
