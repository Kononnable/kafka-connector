use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct OffsetForLeaderEpochResponse {
    pub throttle_time_ms: i32,
    pub topics: Vec<OffsetForLeaderTopicResult>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetForLeaderTopicResult {
    pub name: String,
    pub partitions: Vec<OffsetForLeaderPartitionResult>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetForLeaderPartitionResult {
    pub error_code: i16,
    pub partition_index: i32,
    pub leader_epoch: i32,
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
