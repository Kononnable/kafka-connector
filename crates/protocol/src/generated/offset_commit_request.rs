use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct OffsetCommitRequest {
    pub group_id: String,
    pub generation_id: i32,
    pub member_id: String,
    pub retention_time_ms: i64,
    pub topics: Vec<OffsetCommitRequestTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetCommitRequestTopic {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestPartition>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetCommitRequestPartition {
    pub partition_index: i32,
    pub committed_offset: i64,
    pub committed_leader_epoch: i32,
    pub commit_timestamp: i64,
    pub committed_metadata: String,
}

impl ApiRequest for OffsetCommitRequest {
    type Response = super::offset_commit_response::OffsetCommitResponse;

    fn get_api_key() -> i16 {
        8
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        6
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
        if version >= 1 {
            self.generation_id.serialize(version, bytes);
        }
        if version >= 1 {
            self.member_id.serialize(version, bytes);
        }
        if version >= 2 && version <= 4 {
            self.retention_time_ms.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
    }
}

impl ToBytes for OffsetCommitRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}

impl ToBytes for OffsetCommitRequestPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 0 {
            self.committed_offset.serialize(version, bytes);
        }
        if version >= 6 {
            self.committed_leader_epoch.serialize(version, bytes);
        }
        if version >= 1 {
            self.commit_timestamp.serialize(version, bytes);
        }
        if version >= 0 {
            self.committed_metadata.serialize(version, bytes);
        }
    }
}
