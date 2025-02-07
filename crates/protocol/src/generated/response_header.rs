use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResponseHeader {
    /// The correlation ID of this response.
    pub correlation_id: i32,
}

impl ResponseHeader {
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
        self.correlation_id.serialize(version, _bytes);
        Ok(())
    }

    pub fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let correlation_id = i32::deserialize(version, bytes);
        ResponseHeader { correlation_id }
    }
}

impl ResponseHeader {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
