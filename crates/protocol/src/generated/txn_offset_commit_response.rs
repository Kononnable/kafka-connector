use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct TxnOffsetCommitResponse {
    pub throttle_time_ms: i32,
    pub topics: Vec<TxnOffsetCommitResponseTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct TxnOffsetCommitResponseTopic {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitResponsePartition>,
}

#[derive(Debug, Default, Clone)]
pub struct TxnOffsetCommitResponsePartition {
    pub partition_index: i32,
    pub error_code: i16,
}

impl ApiResponse for TxnOffsetCommitResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<TxnOffsetCommitResponseTopic>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            TxnOffsetCommitResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl FromBytes for TxnOffsetCommitResponseTopic {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<TxnOffsetCommitResponsePartition>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        TxnOffsetCommitResponseTopic { name, partitions }
    }
}

impl FromBytes for TxnOffsetCommitResponsePartition {
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
        TxnOffsetCommitResponsePartition {
            partition_index,
            error_code,
        }
    }
}
