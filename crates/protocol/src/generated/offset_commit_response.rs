use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct OffsetCommitResponse {
    pub throttle_time_ms: i32,
    pub topics: Vec<OffsetCommitResponseTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetCommitResponseTopic {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponsePartition>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetCommitResponsePartition {
    pub partition_index: i32,
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
