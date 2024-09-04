use super::super::prelude::*;

/// Version 1 added the leader epoch to the response.
/// Version 2 added the throttle time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OffsetForLeaderEpochResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each topic we fetched offsets for.
    pub topics: Vec<OffsetForLeaderTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OffsetForLeaderTopicResult {
    /// The topic name.
    pub name: String,

    /// Each partition in the topic we fetched offsets for.
    pub partitions: Vec<OffsetForLeaderPartitionResult>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetForLeaderPartitionResult {
    /// The error code 0, or if there was no error.
    pub error_code: i16,

    /// The partition index.
    pub partition_index: i32,

    /// The leader epoch of the partition.
    pub leader_epoch: i32,

    /// The end offset of the epoch.
    pub end_offset: i64,
}

impl ApiResponse for OffsetForLeaderEpochResponse {
    type Request = super::offset_for_leader_epoch_request::OffsetForLeaderEpochRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(23)
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
        if version >= ApiVersion(2) {
            self.throttle_time_ms.serialize(version, _bytes)?;
        }
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(2) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<OffsetForLeaderTopicResult>::deserialize(version, bytes);
        OffsetForLeaderEpochResponse {
            throttle_time_ms,
            topics,
        }
    }
}

impl OffsetForLeaderEpochResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for OffsetForLeaderTopicResult {
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

impl OffsetForLeaderTopicResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for OffsetForLeaderTopicResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<OffsetForLeaderPartitionResult>::deserialize(version, bytes);
        OffsetForLeaderTopicResult { name, partitions }
    }
}

impl ToBytes for OffsetForLeaderPartitionResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes)?;
        self.partition_index.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.leader_epoch.serialize(version, _bytes)?;
        }
        self.end_offset.serialize(version, _bytes)?;
        Ok(())
    }
}

impl OffsetForLeaderPartitionResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for OffsetForLeaderPartitionResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        let leader_epoch = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let end_offset = i64::deserialize(version, bytes);
        OffsetForLeaderPartitionResult {
            error_code,
            partition_index,
            leader_epoch,
            end_offset,
        }
    }
}

impl Default for OffsetForLeaderPartitionResult {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            partition_index: Default::default(),
            leader_epoch: -1,
            end_offset: Default::default(),
        }
    }
}
