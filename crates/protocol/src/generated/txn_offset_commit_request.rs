use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct TxnOffsetCommitRequest {
    /// The ID of the transaction.
    pub transactional_id: String,

    /// The ID of the group.
    pub group_id: String,

    /// The current producer ID in use by the transactional ID.
    pub producer_id: i64,

    /// The current epoch associated with the producer ID.
    pub producer_epoch: i16,

    /// Each topic that we want to committ offsets for.
    pub topics: Vec<TxnOffsetCommitRequestTopic>,
}

#[derive(Clone, Debug, Default)]
pub struct TxnOffsetCommitRequestTopic {
    /// The topic name.
    pub name: String,

    /// The partitions inside the topic that we want to committ offsets for.
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
}

#[derive(Clone, Debug)]
pub struct TxnOffsetCommitRequestPartition {
    /// The index of the partition within the topic.
    pub partition_index: i32,

    /// The message offset to be committed.
    pub committed_offset: i64,

    /// The leader epoch of the last consumed record.
    pub committed_leader_epoch: i32,

    /// Any associated metadata the client wants to keep.
    pub committed_metadata: Option<String>,
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

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        if version >= 0 {
            self.transactional_id.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.group_id.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.producer_id.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.producer_epoch.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.topics.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl TxnOffsetCommitRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for TxnOffsetCommitRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 0 {
            self.name.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl TxnOffsetCommitRequestTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for TxnOffsetCommitRequestPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 0 {
            self.partition_index.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.committed_offset.serialize(version, bytes)?;
        }
        if version >= 2 {
            self.committed_leader_epoch.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.committed_metadata.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl TxnOffsetCommitRequestPartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.committed_metadata.is_none() && !_version >= 0 {
            return Err(SerializationError::NullValue(
                "committed_metadata",
                _version,
                "TxnOffsetCommitRequestPartition",
            ));
        }
        Ok(())
    }
}

impl Default for TxnOffsetCommitRequestPartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            committed_offset: Default::default(),
            committed_leader_epoch: -1,
            committed_metadata: Default::default(),
        }
    }
}
