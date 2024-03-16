use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeaderAndIsrResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// Each partition.
    pub partitions: Vec<LeaderAndIsrResponsePartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LeaderAndIsrResponsePartition {
    /// The topic name.
    pub topic_name: String,

    /// The partition index.
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for LeaderAndIsrResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let partitions = Vec::<LeaderAndIsrResponsePartition>::deserialize(version, bytes);
        (
            header,
            LeaderAndIsrResponse {
                error_code,
                partitions,
            },
        )
    }
}

impl FromBytes for LeaderAndIsrResponsePartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        LeaderAndIsrResponsePartition {
            topic_name,
            partition_index,
            error_code,
        }
    }
}
