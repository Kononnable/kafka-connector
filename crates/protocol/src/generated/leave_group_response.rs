use super::super::prelude::*;

/// Version 1 adds the throttle time.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeaveGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for LeaveGroupResponse {
    type Request = super::leave_group_request::LeaveGroupRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(13)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(2)
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
        self.error_code.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = i16::deserialize(version, bytes);
        LeaveGroupResponse {
            throttle_time_ms,
            error_code,
        }
    }
}

impl LeaveGroupResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
