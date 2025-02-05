use super::super::prelude::*;

/// Version 1 is the same as version 0.
///
/// Version 2 adds the committed leader epoch.
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TxnOffsetCommitRequestTopic {
    /// The topic name.
    pub name: String,

    /// The partitions inside the topic that we want to committ offsets for.
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
}

#[derive(Clone, Debug, PartialEq)]
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

    fn get_api_key() -> ApiKey {
        ApiKey(28)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(2)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.transactional_id.serialize(version, _bytes)?;
        self.group_id.serialize(version, _bytes)?;
        self.producer_id.serialize(version, _bytes)?;
        self.producer_epoch.serialize(version, _bytes)?;
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let transactional_id = String::deserialize(version, bytes);
        let group_id = String::deserialize(version, bytes);
        let producer_id = i64::deserialize(version, bytes);
        let producer_epoch = i16::deserialize(version, bytes);
        let topics = Vec::<TxnOffsetCommitRequestTopic>::deserialize(version, bytes);
        TxnOffsetCommitRequest {
            transactional_id,
            group_id,
            producer_id,
            producer_epoch,
            topics,
        }
    }
}

impl TxnOffsetCommitRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for TxnOffsetCommitRequestTopic {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        self.partitions.serialize(version, _bytes)?;
        Ok(())
    }
}

impl TxnOffsetCommitRequestTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for TxnOffsetCommitRequestTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<TxnOffsetCommitRequestPartition>::deserialize(version, bytes);
        TxnOffsetCommitRequestTopic { name, partitions }
    }
}

impl ToBytes for TxnOffsetCommitRequestPartition {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, _bytes)?;
        self.committed_offset.serialize(version, _bytes)?;
        if version >= ApiVersion(2) {
            self.committed_leader_epoch.serialize(version, _bytes)?;
        }
        self.committed_metadata.serialize(version, _bytes)?;
        Ok(())
    }
}

impl TxnOffsetCommitRequestPartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for TxnOffsetCommitRequestPartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let committed_offset = i64::deserialize(version, bytes);
        let committed_leader_epoch = if version >= ApiVersion(2) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let committed_metadata = Option::<String>::deserialize(version, bytes);
        TxnOffsetCommitRequestPartition {
            partition_index,
            committed_offset,
            committed_leader_epoch,
            committed_metadata,
        }
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
