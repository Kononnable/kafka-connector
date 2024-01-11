use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DescribeLogDirsRequest {
    /// Each topic that we want to describe log directories for, or null for all topics.
    pub topics: Vec<DescribableLogDirTopic>,
}

#[derive(Debug, Clone)]
pub struct DescribableLogDirTopic {
    /// The topic name
    pub topic: String,

    /// The partition indxes.
    pub partition_index: Vec<i32>,
}

impl ApiRequest for DescribeLogDirsRequest {
    type Response = super::describe_log_dirs_response::DescribeLogDirsResponse;

    fn get_api_key() -> i16 {
        35
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
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
    }
}

impl Default for DescribeLogDirsRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
        }
    }
}

impl ToBytes for DescribableLogDirTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.topic.serialize(version, bytes);
        }
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
    }
}

impl Default for DescribableLogDirTopic {
    fn default() -> Self {
        Self {
            topic: Default::default(),
            partition_index: Default::default(),
        }
    }
}
