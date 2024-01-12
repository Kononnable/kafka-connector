use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AlterReplicaLogDirsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub results: Vec<AlterReplicaLogDirTopicResult>,
}

#[derive(Clone, Debug, Default)]
pub struct AlterReplicaLogDirTopicResult {
    /// The name of the topic.
    pub topic_name: String,

    /// The results for each partition.
    pub partitions: Vec<AlterReplicaLogDirPartitionResult>,
}

#[derive(Clone, Debug, Default)]
pub struct AlterReplicaLogDirPartitionResult {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for AlterReplicaLogDirsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<AlterReplicaLogDirTopicResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
        let topic_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<AlterReplicaLogDirPartitionResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        AlterReplicaLogDirTopicResult {
            topic_name,
            partitions,
        }
    }
}

impl FromBytes for AlterReplicaLogDirPartitionResult {
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
        AlterReplicaLogDirPartitionResult {
            partition_index,
            error_code,
        }
    }
}
