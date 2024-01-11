use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ListOffsetRequest {
    pub replica_id: i32,
    pub isolation_level: i8,
    pub topics: Vec<ListOffsetTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct ListOffsetTopic {
    pub name: String,
    pub partitions: Vec<ListOffsetPartition>,
}

#[derive(Debug, Default, Clone)]
pub struct ListOffsetPartition {
    pub partition_index: i32,
    pub current_leader_epoch: i32,
    pub timestamp: i64,
    pub max_num_offsets: i32,
}

impl ApiRequest for ListOffsetRequest {
    type Response = super::list_offset_response::ListOffsetResponse;

    fn get_api_key() -> i16 {
        2
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
            self.replica_id.serialize(version, bytes);
        }
        if version >= 2 {
            self.isolation_level.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
    }
}

impl ToBytes for ListOffsetTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}

impl ToBytes for ListOffsetPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 4 {
            self.current_leader_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.timestamp.serialize(version, bytes);
        }
        if version >= 0 {
            self.max_num_offsets.serialize(version, bytes);
        }
    }
}
