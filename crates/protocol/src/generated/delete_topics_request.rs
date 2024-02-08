use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteTopicsRequest {
    /// The names of the topics to delete
    pub topic_names: Vec<String>,

    /// The length of time in milliseconds to wait for the deletions to complete.
    pub timeout_ms: i32,
}

impl ApiRequest for DeleteTopicsRequest {
    type Response = super::delete_topics_response::DeleteTopicsResponse;

    fn get_api_key() -> i16 {
        20
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        3
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
        self.topic_names.serialize(version, bytes)?;
        self.timeout_ms.serialize(version, bytes)?;
        Ok(())
    }
}

impl DeleteTopicsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
