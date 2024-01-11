use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct StopReplicaResponse {
    pub error_code: i16,
    pub partitions: Vec<StopReplicaResponsePartition>,
}

#[derive(Debug, Default, Clone)]
pub struct StopReplicaResponsePartition {
    pub topic_name: String,
    pub partition_index: i32,
    pub error_code: i16,
}

impl ApiResponse for StopReplicaResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<StopReplicaResponsePartition>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            StopReplicaResponse {
                error_code,
                partitions,
            },
        )
    }
}

impl FromBytes for StopReplicaResponsePartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let topic_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
        StopReplicaResponsePartition {
            topic_name,
            partition_index,
            error_code,
        }
    }
}
