use super::super::prelude::*;

/// Version 1 adds throttle time and error messages.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FindCoordinatorResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The error message, or null if there was no error.
    pub error_message: Option<String>,

    /// The node id.
    pub node_id: i32,

    /// The host name.
    pub host: String,

    /// The port.
    pub port: i32,
}

impl ApiResponse for FindCoordinatorResponse {
    type Request = super::find_coordinator_request::FindCoordinatorRequest;

    fn get_api_key() -> i16 {
        10
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
        if version >= 1 {
            self.error_message.serialize(version, bytes)?;
        }
        self.node_id.serialize(version, bytes)?;
        self.host.serialize(version, bytes)?;
        self.port.serialize(version, bytes)?;
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
        let error_message = if version >= 1 {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let node_id = i32::deserialize(version, bytes);
        let host = String::deserialize(version, bytes);
        let port = i32::deserialize(version, bytes);
        (
            header,
            FindCoordinatorResponse {
                throttle_time_ms,
                error_code,
                error_message,
                node_id,
                host,
                port,
            },
        )
    }
}

impl FindCoordinatorResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.error_message.is_none() && !_version >= 1 {
            return Err(SerializationError::NullValue(
                "error_message",
                _version,
                "FindCoordinatorResponse",
            ));
        }
        Ok(())
    }
}
