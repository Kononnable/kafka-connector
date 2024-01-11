use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct OffsetForLeaderEpochRequest {
    pub topics: Vec<OffsetForLeaderTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetForLeaderTopic {
    pub name: String,
    pub partitions: Vec<OffsetForLeaderPartition>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetForLeaderPartition {
    pub partition_index: i32,
    pub current_leader_epoch: i32,
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
