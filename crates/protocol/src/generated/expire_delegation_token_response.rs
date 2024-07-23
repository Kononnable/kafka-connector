use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpireDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The timestamp in milliseconds at which this token expires.
    pub expiry_timestamp_ms: i64,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

impl ApiResponse for ExpireDelegationTokenResponse {
    type Request = super::expire_delegation_token_request::ExpireDelegationTokenRequest;

    fn get_api_key() -> i16 {
        40
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
        self.error_code.serialize(version, bytes)?;
        self.expiry_timestamp_ms.serialize(version, bytes)?;
        self.throttle_time_ms.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let expiry_timestamp_ms = i64::deserialize(version, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        (
            header,
            ExpireDelegationTokenResponse {
                error_code,
                expiry_timestamp_ms,
                throttle_time_ms,
            },
        )
    }
}

impl ExpireDelegationTokenResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
