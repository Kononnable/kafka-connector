use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct OffsetCommitResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses for each topic.
    pub topics: Vec<OffsetCommitResponseTopic>,
}

#[derive(Debug, Clone)]
pub struct OffsetCommitResponseTopic {
    /// The topic name.
    pub name: String,

    /// The responses for each partition in the topic.
    pub partitions: Vec<OffsetCommitResponsePartition>,
}

#[derive(Debug, Clone)]
pub struct OffsetCommitResponsePartition {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for OffsetCommitResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 3 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<OffsetCommitResponseTopic>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            OffsetCommitResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl Default for OffsetCommitResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            topics: Default::default(),
        }
    }
}

impl FromBytes for OffsetCommitResponseTopic {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<OffsetCommitResponsePartition>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        OffsetCommitResponseTopic { name, partitions }
    }
}

impl Default for OffsetCommitResponseTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl FromBytes for OffsetCommitResponsePartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        OffsetCommitResponsePartition {
            partition_index,
            error_code,
        }
    }
}

impl Default for OffsetCommitResponsePartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            error_code: Default::default(),
        }
    }
}