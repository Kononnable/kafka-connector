use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct FindCoordinatorRequest {
    /// The coordinator key.
    pub key: String,

    /// The coordinator key type.  (Group, transaction, etc.)
    pub key_type: i8,
}

impl ApiRequest for FindCoordinatorRequest {
    type Response = super::find_coordinator_response::FindCoordinatorResponse;

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
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        if version >= 0 {
            self.key.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.key_type.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl FindCoordinatorRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
