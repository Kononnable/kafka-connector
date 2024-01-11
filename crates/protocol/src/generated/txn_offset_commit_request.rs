use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct TxnOffsetCommitRequest {
    pub transactional_id: String,
    pub group_id: String,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub topics: Vec<TxnOffsetCommitRequestTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct TxnOffsetCommitRequestTopic {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
}

#[derive(Debug, Default, Clone)]
pub struct TxnOffsetCommitRequestPartition {
    pub partition_index: i32,
    pub committed_offset: i64,
    pub committed_leader_epoch: i32,
    pub committed_metadata: String,
}

impl ApiRequest for TxnOffsetCommitRequest {
    type Response = super::txn_offset_commit_response::TxnOffsetCommitResponse;

    fn get_api_key() -> i16 {
        28
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
            self.transactional_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.group_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.producer_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.producer_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
    }
}

impl ToBytes for TxnOffsetCommitRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}

impl ToBytes for TxnOffsetCommitRequestPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 0 {
            self.committed_offset.serialize(version, bytes);
        }
        if version >= 2 {
            self.committed_leader_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.committed_metadata.serialize(version, bytes);
        }
    }
}
