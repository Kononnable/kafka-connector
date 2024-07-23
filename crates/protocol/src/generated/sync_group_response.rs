use super::super::prelude::*;

/// Version 1 adds throttle time.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SyncGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The member assignment.
    pub assignment: Vec<u8>,
}

impl ApiResponse for SyncGroupResponse {
    type Request = super::sync_group_request::SyncGroupRequest;

    fn get_api_key() -> i16 {
        14
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
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
        self.error_code.serialize(version, bytes)?;
        self.assignment.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = i16::deserialize(version, bytes);
        let assignment = Vec::<u8>::deserialize(version, bytes);
        (
            header,
            SyncGroupResponse {
                throttle_time_ms,
                error_code,
                assignment,
            },
        )
    }
}

impl SyncGroupResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
