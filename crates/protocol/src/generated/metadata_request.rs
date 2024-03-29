use super::super::prelude::*;

/// In version 0, an empty array indicates "request metadata for all topics."  In version 1 and
/// higher, an empty array indicates "request metadata for no topics," and a null array is used to
/// indiate "request metadata for all topics."
///
/// Version 2 and 3 are the same as version 1.
///
/// Version 4 adds AllowAutoTopicCreation.
#[derive(Clone, Debug, PartialEq)]
pub struct MetadataRequest {
    /// The topics to fetch metadata for.
    pub topics: Option<Vec<MetadataRequestTopic>>,

    /// If this is true, the broker may auto-create topics that we requested which do not already exist, if it is configured to do so.
    pub allow_auto_topic_creation: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MetadataRequestTopic {
    /// The topic name.
    pub name: String,
}

impl ApiRequest for MetadataRequest {
    type Response = super::metadata_response::MetadataResponse;

    fn get_api_key() -> i16 {
        3
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        7
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
        self.topics.serialize(version, bytes)?;
        if version >= 4 {
            self.allow_auto_topic_creation.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl MetadataRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topics.is_none() {
            return Err(SerializationError::NullValue(
                "topics",
                _version,
                "MetadataRequest",
            ));
        }
        if self.allow_auto_topic_creation != bool::default() && _version >= 4 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "allow_auto_topic_creation",
                _version,
                "MetadataRequest",
            ));
        }
        Ok(())
    }
}

impl Default for MetadataRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            allow_auto_topic_creation: true,
        }
    }
}

impl ToBytes for MetadataRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl MetadataRequestTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
