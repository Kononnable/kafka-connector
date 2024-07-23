use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreateAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each ACL creation.
    pub results: Vec<CreatableAclResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatableAclResult {
    /// The result error, or zero if there was no error.
    pub error_code: i16,

    /// The result message, or null if there was no error.
    pub error_message: Option<String>,
}

impl ApiResponse for CreateAclsResponse {
    type Request = super::create_acls_request::CreateAclsRequest;

    fn get_api_key() -> i16 {
        30
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
        self.results.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<CreatableAclResult>::deserialize(version, bytes);
        (
            header,
            CreateAclsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl CreateAclsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatableAclResult {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, bytes)?;
        self.error_message.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreatableAclResult {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.error_message.is_none() {
            return Err(SerializationError::NullValue(
                "error_message",
                _version,
                "CreatableAclResult",
            ));
        }
        Ok(())
    }
}

impl FromBytes for CreatableAclResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        CreatableAclResult {
            error_code,
            error_message,
        }
    }
}
