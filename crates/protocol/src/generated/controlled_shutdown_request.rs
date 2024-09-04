use super::super::prelude::*;

/// Version 0 of ControlledShutdownRequest has a non-standard request header
/// which does not include clientId.  Version 1 and later use the standard
/// request header.
///
/// Version 1 is the same as version 0.
///
/// Version 2 adds BrokerEpoch.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlledShutdownRequest {
    /// The id of the broker for which controlled shutdown has been requested.
    pub broker_id: i32,

    /// The broker epoch.
    pub broker_epoch: i64,
}

impl ApiRequest for ControlledShutdownRequest {
    type Response = super::controlled_shutdown_response::ControlledShutdownResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(7)
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
        self.broker_id.serialize(version, _bytes)?;
        if version >= ApiVersion(2) {
            self.broker_epoch.serialize(version, _bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let broker_id = i32::deserialize(version, bytes);
        let broker_epoch = if version >= ApiVersion(2) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ControlledShutdownRequest {
            broker_id,
            broker_epoch,
        }
    }
}

impl ControlledShutdownRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl Default for ControlledShutdownRequest {
    fn default() -> Self {
        Self {
            broker_id: Default::default(),
            broker_epoch: -1,
        }
    }
}
