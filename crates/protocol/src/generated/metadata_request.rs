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

    fn get_api_key() -> ApiKey {
        ApiKey(3)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(7)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.topics.serialize(version, _bytes);
        if version >= ApiVersion(4) {
            self.allow_auto_topic_creation.serialize(version, _bytes);
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topics = Option::<Vec<MetadataRequestTopic>>::deserialize(version, bytes);
        let allow_auto_topic_creation = if version >= ApiVersion(4) {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        MetadataRequest {
            topics,
            allow_auto_topic_creation,
        }
    }
}

impl MetadataRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter().flatten() {
            item.validate_fields(_version)?;
        }
        if self.topics.is_none() && !_version.0 < 1 {
            return Err(SerializationError::NullValue(
                "topics",
                *_version,
                "MetadataRequest",
            ));
        }
        if self.allow_auto_topic_creation != true && _version.0 < 4 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "allow_auto_topic_creation",
                *_version,
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
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
    }
}

impl MetadataRequestTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataRequestTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        MetadataRequestTopic { name }
    }
}
