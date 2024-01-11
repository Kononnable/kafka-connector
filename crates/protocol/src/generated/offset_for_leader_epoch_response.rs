use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct OffsetForLeaderEpochResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each topic we fetched offsets for.
    pub topics: Vec<OffsetForLeaderTopicResult>,
}

#[derive(Debug, Clone)]
pub struct OffsetForLeaderTopicResult {
    /// The topic name.
    pub name: String,

    /// Each partition in the topic we fetched offsets for.
    pub partitions: Vec<OffsetForLeaderPartitionResult>,
}

#[derive(Debug, Clone)]
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
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 2 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<OffsetForLeaderTopicResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            OffsetForLeaderEpochResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl Default for OffsetForLeaderEpochResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            topics: Default::default(),
        }
    }
}

impl FromBytes for OffsetForLeaderTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<OffsetForLeaderPartitionResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        OffsetForLeaderTopicResult { name, partitions }
    }
}

impl Default for OffsetForLeaderTopicResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl FromBytes for OffsetForLeaderPartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let end_offset = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
