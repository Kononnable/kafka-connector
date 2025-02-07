use super::super::prelude::*;

/// Versions 0, 1, 2, and 3 are the same.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteTopicsRequest {
    /// The names of the topics to delete
    pub topic_names: Vec<String>,

    /// The length of time in milliseconds to wait for the deletions to complete.
    pub timeout_ms: i32,
}

impl ApiRequest for DeleteTopicsRequest {
    type Response = super::delete_topics_response::DeleteTopicsResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(20)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(3)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.topic_names.serialize(version, _bytes);
        self.timeout_ms.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic_names = Vec::<String>::deserialize(version, bytes);
        let timeout_ms = i32::deserialize(version, bytes);
        DeleteTopicsRequest {
            topic_names,
            timeout_ms,
        }
    }
}

impl DeleteTopicsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
