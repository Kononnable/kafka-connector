use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct TxnOffsetCommitResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses for each topic.
    pub topics: Vec<TxnOffsetCommitResponseTopic>,
}

#[derive(Clone, Debug, Default)]
pub struct TxnOffsetCommitResponseTopic {
    /// The topic name.
    pub name: String,

    /// The responses for each partition in the topic.
    pub partitions: Vec<TxnOffsetCommitResponsePartition>,
}

#[derive(Clone, Debug, Default)]
pub struct TxnOffsetCommitResponsePartition {
    /// The partitition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for TxnOffsetCommitResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let topics = Vec::<TxnOffsetCommitResponseTopic>::deserialize(version, bytes);
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
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<TxnOffsetCommitResponsePartition>::deserialize(version, bytes);
        TxnOffsetCommitResponseTopic { name, partitions }
    }
}

impl FromBytes for TxnOffsetCommitResponsePartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        TxnOffsetCommitResponsePartition {
            partition_index,
            error_code,
        }
    }
}
