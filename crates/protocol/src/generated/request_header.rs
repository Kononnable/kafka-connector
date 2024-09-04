use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
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
    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    pub fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.request_api_key.serialize(version, _bytes)?;
        self.request_api_version.serialize(version, _bytes)?;
        self.correlation_id.serialize(version, _bytes)?;
        self.client_id.serialize(version, _bytes)?;
        Ok(())
    }

    pub fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let request_api_key = i16::deserialize(version, bytes);
        let request_api_version = i16::deserialize(version, bytes);
        let correlation_id = i32::deserialize(version, bytes);
        let client_id = String::deserialize(version, bytes);
        RequestHeader {
            request_api_key,
            request_api_version,
            correlation_id,
            client_id,
        }
    }
}

impl RequestHeader {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
