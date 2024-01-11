use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AlterReplicaLogDirsResponse {
    pub throttle_time_ms: i32,
    pub results: Vec<AlterReplicaLogDirTopicResult>,
}

#[derive(Debug, Default, Clone)]
pub struct AlterReplicaLogDirTopicResult {
    pub topic_name: String,
    pub partitions: Vec<AlterReplicaLogDirPartitionResult>,
}

#[derive(Debug, Default, Clone)]
pub struct AlterReplicaLogDirPartitionResult {
    pub partition_index: i32,
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
