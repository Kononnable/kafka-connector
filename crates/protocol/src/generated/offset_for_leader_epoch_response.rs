use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct OffsetForLeaderEpochResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each topic we fetched offsets for.
    pub topics: Vec<OffsetForLeaderTopicResult>,
}

#[derive(Clone, Debug, Default)]
pub struct OffsetForLeaderTopicResult {
    /// The topic name.
    pub name: String,

    /// Each partition in the topic we fetched offsets for.
    pub partitions: Vec<OffsetForLeaderPartitionResult>,
}

#[derive(Clone, Debug)]
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
        let topics = Vec::<OffsetForLeaderTopicResult>::deserialize(version, bytes);
        (
            header,
            OffsetForLeaderEpochResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl FromBytes for OffsetForLeaderTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<OffsetForLeaderPartitionResult>::deserialize(version, bytes);
        OffsetForLeaderTopicResult { name, partitions }
    }
}

impl FromBytes for OffsetForLeaderPartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        let leader_epoch = if version >= 1 {
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
