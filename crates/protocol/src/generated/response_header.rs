use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResponseHeader {
    /// The correlation ID of this response.
    pub correlation_id: i32,
}

impl ResponseHeader {
    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        0
    }

    pub fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.correlation_id.serialize(version, bytes)?;
        Ok(())
    }

    pub fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let correlation_id = i32::deserialize(version, bytes);
        ResponseHeader { correlation_id }
    }
}

impl ResponseHeader {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
