use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct ElectPreferredLeadersRequest {
    /// The topic partitions to elect the preferred leader of.
    pub topic_partitions: Vec<TopicPartitions>,

    /// The time in ms to wait for the election to complete.
    pub timeout_ms: i32,
}

#[derive(Debug, Clone)]
pub struct TopicPartitions {
    /// The name of a topic.
    pub topic: String,

    /// The partitions of this topic whose preferred leader should be elected
    pub partition_id: Vec<i32>,
}

impl ApiRequest for ElectPreferredLeadersRequest {
    type Response = super::elect_preferred_leaders_response::ElectPreferredLeadersResponse;

    fn get_api_key() -> i16 {
        43
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        0
    }

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.topic_partitions.serialize(version, bytes);
        }
        if version >= 0 {
            self.timeout_ms.serialize(version, bytes);
        }
    }
}

impl Default for ElectPreferredLeadersRequest {
    fn default() -> Self {
        Self {
            topic_partitions: Default::default(),
            timeout_ms: 60000,
        }
    }
}

impl ToBytes for TopicPartitions {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.topic.serialize(version, bytes);
        }
        if version >= 0 {
            self.partition_id.serialize(version, bytes);
        }
    }
}

impl Default for TopicPartitions {
    fn default() -> Self {
        Self {
            topic: Default::default(),
            partition_id: Default::default(),
        }
    }
}
