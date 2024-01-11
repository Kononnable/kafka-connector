use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct OffsetFetchRequest {
    /// The group to fetch offsets for.
    pub group_id: String,

    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    pub topics: Vec<OffsetFetchRequestTopic>,
}

#[derive(Debug, Clone)]
pub struct OffsetFetchRequestTopic {
    pub name: String,

    /// The partition indexes we would like to fetch offsets for.
    pub partition_indexes: Vec<i32>,
}

impl ApiRequest for OffsetFetchRequest {
    type Response = super::offset_fetch_response::OffsetFetchResponse;

    fn get_api_key() -> i16 {
        9
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        5
    }

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.group_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
    }
}

impl Default for OffsetFetchRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            topics: Default::default(),
        }
    }
}

impl ToBytes for OffsetFetchRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partition_indexes.serialize(version, bytes);
        }
    }
}

impl Default for OffsetFetchRequestTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_indexes: Default::default(),
        }
    }
}
