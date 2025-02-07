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

    fn get_api_key() -> ApiKey {
        ApiKey(8)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(6)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.group_id.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.generation_id.serialize(version, _bytes);
        }
        if version >= ApiVersion(1) {
            self.member_id.serialize(version, _bytes);
        }
        if (2..=4).contains(&version.0) {
            self.retention_time_ms.serialize(version, _bytes);
        }
        self.topics.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let generation_id = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let member_id = if version >= ApiVersion(1) {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let retention_time_ms = if (2..=4).contains(&version.0) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<OffsetCommitRequestTopic>::deserialize(version, bytes);
        OffsetCommitRequest {
            group_id,
            generation_id,
            member_id,
            retention_time_ms,
            topics,
        }
    }
}

impl OffsetCommitRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.validate_fields(_version)?;
        }
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
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.partitions.serialize(version, _bytes);
    }
}

impl OffsetCommitRequestTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.partitions.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for OffsetCommitRequestTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<OffsetCommitRequestPartition>::deserialize(version, bytes);
        OffsetCommitRequestTopic { name, partitions }
    }
}

impl ToBytes for OffsetCommitRequestPartition {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.partition_index.serialize(version, _bytes);
        self.committed_offset.serialize(version, _bytes);
        if version >= ApiVersion(6) {
            self.committed_leader_epoch.serialize(version, _bytes);
        }
        if version >= ApiVersion(1) {
            self.commit_timestamp.serialize(version, _bytes);
        }
        self.committed_metadata.serialize(version, _bytes);
    }
}

impl OffsetCommitRequestPartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.commit_timestamp != -1 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "commit_timestamp",
                *_version,
                "OffsetCommitRequestPartition",
            ));
        }
        Ok(())
    }
}

impl FromBytes for OffsetCommitRequestPartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let committed_offset = i64::deserialize(version, bytes);
        let committed_leader_epoch = if version >= ApiVersion(6) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let commit_timestamp = if version >= ApiVersion(1) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let committed_metadata = Option::<String>::deserialize(version, bytes);
        OffsetCommitRequestPartition {
            partition_index,
            committed_offset,
            committed_leader_epoch,
            commit_timestamp,
            committed_metadata,
        }
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
