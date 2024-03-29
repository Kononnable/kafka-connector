use super::super::prelude::*;

/// Version 1 adds timestamp and group membership information, as well as the commit timestamp.
///
/// Version 2 adds retention time.  It removes the commit timestamp added in version 1.
///
/// Version 3 and 4 are the same as version 2.
///
/// Version 5 removes the retention time, which is now controlled only by a broker configuration.
///
/// Version 6 adds the leader epoch for fencing.
/// CommitTimestamp has been removed from v2 and later.
#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCommitRequest {
    /// The unique group identifier.
    pub group_id: String,

    /// The generation of the group.
    pub generation_id: i32,

    /// The member ID assigned by the group coordinator.
    pub member_id: String,

    /// The time period in ms to retain the offset.
    pub retention_time_ms: i64,

    /// The topics to commit offsets for.
    pub topics: Vec<OffsetCommitRequestTopic>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OffsetCommitRequestTopic {
    /// The topic name.
    pub name: String,

    /// Each partition to commit offsets for.
    pub partitions: Vec<OffsetCommitRequestPartition>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCommitRequestPartition {
    /// The partition index.
    pub partition_index: i32,

    /// The message offset to be committed.
    pub committed_offset: i64,

    /// The leader epoch of this partition.
    pub committed_leader_epoch: i32,

    /// The timestamp of the commit.
    pub commit_timestamp: i64,

    /// Any associated metadata the client wants to keep.
    pub committed_metadata: Option<String>,
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
        self.group_id.serialize(version, bytes)?;
        if version >= 1 {
            self.generation_id.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.member_id.serialize(version, bytes)?;
        }
        if (2..=4).contains(&version) {
            self.retention_time_ms.serialize(version, bytes)?;
        }
        self.topics.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetCommitRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl Default for OffsetCommitRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            generation_id: -1,
            member_id: Default::default(),
            retention_time_ms: -1,
            topics: Default::default(),
        }
    }
}

impl ToBytes for OffsetCommitRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetCommitRequestTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for OffsetCommitRequestPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.committed_offset.serialize(version, bytes)?;
        if version >= 6 {
            self.committed_leader_epoch.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.commit_timestamp.serialize(version, bytes)?;
        }
        self.committed_metadata.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetCommitRequestPartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.committed_metadata.is_none() {
            return Err(SerializationError::NullValue(
                "committed_metadata",
                _version,
                "OffsetCommitRequestPartition",
            ));
        }
        if self.commit_timestamp != i64::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "commit_timestamp",
                _version,
                "OffsetCommitRequestPartition",
            ));
        }
        Ok(())
    }
}

impl Default for OffsetCommitRequestPartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            committed_offset: Default::default(),
            committed_leader_epoch: -1,
            commit_timestamp: -1,
            committed_metadata: Default::default(),
        }
    }
}
