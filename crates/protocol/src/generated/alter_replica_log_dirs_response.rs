use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlterReplicaLogDirsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub results: Vec<AlterReplicaLogDirTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterReplicaLogDirTopicResult {
    /// The name of the topic.
    pub topic_name: String,

    /// The results for each partition.
    pub partitions: Vec<AlterReplicaLogDirPartitionResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterReplicaLogDirPartitionResult {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for AlterReplicaLogDirsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<AlterReplicaLogDirTopicResult>::deserialize(version, bytes);
        (
            header,
            AlterReplicaLogDirsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for AlterReplicaLogDirTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partitions = Vec::<AlterReplicaLogDirPartitionResult>::deserialize(version, bytes);
        AlterReplicaLogDirTopicResult {
            topic_name,
            partitions,
        }
    }
}

impl FromBytes for AlterReplicaLogDirPartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        AlterReplicaLogDirPartitionResult {
            partition_index,
            error_code,
        }
    }
}
