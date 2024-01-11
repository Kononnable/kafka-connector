use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct MetadataRequest {
    /// The topics to fetch metadata for.
    pub topics: Vec<MetadataRequestTopic>,

    /// If this is true, the broker may auto-create topics that we requested which do not already exist, if it is configured to do so.
    pub allow_auto_topic_creation: bool,
}

#[derive(Debug, Clone)]
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
        if version >= 4 {
            self.allow_auto_topic_creation.serialize(version, bytes);
        }
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
    }
}

impl Default for MetadataRequestTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
        }
    }
}