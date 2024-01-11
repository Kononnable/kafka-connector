use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ElectPreferredLeadersRequest {
    pub topic_partitions: Vec<TopicPartitions>,
    pub timeout_ms: i32,
}

#[derive(Debug, Default, Clone)]
pub struct TopicPartitions {
    pub topic: String,
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
