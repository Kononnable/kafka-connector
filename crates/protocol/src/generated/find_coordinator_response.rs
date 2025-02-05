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

    fn get_api_key() -> ApiKey {
        ApiKey(10)
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
            self.throttle_time_ms.serialize(version, _bytes)?;
        }
        self.error_code.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.error_message.serialize(version, _bytes)?;
        }
        self.node_id.serialize(version, _bytes)?;
        self.host.serialize(version, _bytes)?;
        self.port.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = i16::deserialize(version, bytes);
        let error_message = if version >= ApiVersion(1) {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let node_id = i32::deserialize(version, bytes);
        let host = String::deserialize(version, bytes);
        let port = i32::deserialize(version, bytes);
        FindCoordinatorResponse {
            throttle_time_ms,
            error_code,
            error_message,
            node_id,
            host,
            port,
        }
    }
}

impl FindCoordinatorResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
