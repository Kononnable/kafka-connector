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
        self.key.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.key_type.serialize(version, _bytes);
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let key = String::deserialize(version, bytes);
        let key_type = if version >= ApiVersion(1) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        FindCoordinatorRequest { key, key_type }
    }
}

impl FindCoordinatorRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.key_type != 0 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "key_type",
                *_version,
                "FindCoordinatorRequest",
            ));
        }
        Ok(())
    }
}
