use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct OffsetForLeaderEpochRequest {
    /// Each topic to get offsets for.
    pub topics: Vec<OffsetForLeaderTopic>,
}

#[derive(Debug, Clone)]
pub struct OffsetForLeaderTopic {
    /// The topic name.
    pub name: String,

    /// Each partition to get offsets for.
    pub partitions: Vec<OffsetForLeaderPartition>,
}

#[derive(Debug, Clone)]
pub struct OffsetForLeaderPartition {
    /// The partition index.
    pub partition_index: i32,

    /// An epoch used to fence consumers/replicas with old metadata.  If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
    pub current_leader_epoch: i32,

    /// The epoch to look up an offset for.
    pub leader_epoch: i32,
}

impl ApiRequest for OffsetForLeaderEpochRequest {
    type Response = super::offset_for_leader_epoch_response::OffsetForLeaderEpochResponse;

    fn get_api_key() -> i16 {
        23
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
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

impl Default for OffsetForLeaderEpochRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
        }
    }
}

impl ToBytes for OffsetForLeaderTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}

impl Default for OffsetForLeaderTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl ToBytes for OffsetForLeaderPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 2 {
            self.current_leader_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.leader_epoch.serialize(version, bytes);
        }
    }
}

impl Default for OffsetForLeaderPartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            current_leader_epoch: -1,
            leader_epoch: Default::default(),
        }
    }
}