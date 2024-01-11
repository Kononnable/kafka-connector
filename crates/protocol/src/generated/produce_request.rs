use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ProduceRequest {
    pub transactional_id: String,
    pub acks: i16,
    pub timeout_ms: i32,
    pub topics: Vec<TopicProduceData>,
}

#[derive(Debug, Default, Clone)]
pub struct TopicProduceData {
    pub name: String,
    pub partitions: Vec<PartitionProduceData>,
}

#[derive(Debug, Default, Clone)]
pub struct PartitionProduceData {
    pub partition_index: i32,
    pub records: Vec<u8>,
}

impl ApiRequest for ProduceRequest {
    type Response = super::produce_response::ProduceResponse;

    fn get_api_key() -> i16 {
        0
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
        if version >= 3 {
            self.transactional_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.acks.serialize(version, bytes);
        }
        if version >= 0 {
            self.timeout_ms.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
    }
}

impl ToBytes for TopicProduceData {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}

impl ToBytes for PartitionProduceData {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 0 {
            self.records.serialize(version, bytes);
        }
    }
}
