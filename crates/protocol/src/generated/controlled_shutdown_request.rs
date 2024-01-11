use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct ControlledShutdownRequest {
    /// The id of the broker for which controlled shutdown has been requested.
    pub broker_id: i32,

    /// The broker epoch.
    pub broker_epoch: i64,
}

impl ApiRequest for ControlledShutdownRequest {
    type Response = super::controlled_shutdown_response::ControlledShutdownResponse;

    fn get_api_key() -> i16 {
        7
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
    }

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.broker_id.serialize(version, bytes);
        }
        if version >= 2 {
            self.broker_epoch.serialize(version, bytes);
        }
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
