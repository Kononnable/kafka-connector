use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct RequestHeader {
    /// The API key of this request.
    pub request_api_key: i16,

    /// The API version of this request.
    pub request_api_version: i16,

    /// The correlation ID of this request.
    pub correlation_id: i32,

    /// The client ID string.
    pub client_id: String,
}

impl RequestHeader {
    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        0
    }

    pub fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        if version >= 0 {
            self.request_api_key.serialize(version, bytes);
        }
        if version >= 0 {
            self.request_api_version.serialize(version, bytes);
        }
        if version >= 0 {
            self.correlation_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.client_id.serialize(version, bytes);
        }
    }
}

impl Default for RequestHeader {
    fn default() -> Self {
        Self {
            request_api_key: Default::default(),
            request_api_version: Default::default(),
            correlation_id: Default::default(),
            client_id: Default::default(),
        }
    }
}