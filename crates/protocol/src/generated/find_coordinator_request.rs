use super::super::prelude::*;

/// Version 1 adds KeyType.
/// Version 2 is the same as version 1.
#[derive(Clone, Debug, Default, PartialEq)]
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
        self.key.serialize(version, bytes)?;
        if version >= 1 {
            self.key_type.serialize(version, bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let key = String::deserialize(version, bytes);
        let key_type = if version >= 1 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        FindCoordinatorRequest { key, key_type }
    }
}

impl FindCoordinatorRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.key_type != i8::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "key_type",
                _version,
                "FindCoordinatorRequest",
            ));
        }
        Ok(())
    }
}
