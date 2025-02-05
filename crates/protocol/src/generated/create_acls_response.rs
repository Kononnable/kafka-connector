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

    fn get_api_key() -> ApiKey {
        ApiKey(30)
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
        let results = Vec::<CreatableAclResult>::deserialize(version, bytes);
        CreateAclsResponse {
            throttle_time_ms,
            results,
        }
    }
}

impl CreateAclsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatableAclResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes)?;
        self.error_message.serialize(version, _bytes)?;
        Ok(())
    }
}

impl CreatableAclResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatableAclResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        CreatableAclResult {
            error_code,
            error_message,
        }
    }
}
