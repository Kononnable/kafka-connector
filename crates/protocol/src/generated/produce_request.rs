use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct ProduceRequest {
    /// The transactional ID, or null if the producer is not transactional.
    pub transactional_id: String,

    /// The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
    pub acks: i16,

    /// The timeout to await a response in miliseconds.
    pub timeout_ms: i32,

    /// Each topic to produce to.
    pub topics: Vec<TopicProduceData>,
}

#[derive(Debug, Clone)]
pub struct TopicProduceData {
    /// The topic name.
    pub name: String,

    /// Each partition to produce to.
    pub partitions: Vec<PartitionProduceData>,
}

#[derive(Debug, Clone)]
pub struct PartitionProduceData {
    /// The partition index.
    pub partition_index: i32,

    /// The record data to be produced.
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

impl Default for ProduceRequest {
    fn default() -> Self {
        Self {
            transactional_id: Default::default(),
            acks: Default::default(),
            timeout_ms: Default::default(),
            topics: Default::default(),
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

impl Default for TopicProduceData {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
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

impl Default for PartitionProduceData {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            records: Default::default(),
        }
    }
}
