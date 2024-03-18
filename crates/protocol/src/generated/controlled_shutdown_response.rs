use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ControlledShutdownResponse {
    /// The top-level error code.
    pub error_code: i16,

    /// The partitions that the broker still leads.
    pub remaining_partitions: IndexSet<RemainingPartition>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct RemainingPartition {
    /// The name of the topic.
    pub topic_name: String,

    /// The index of the partition.
    pub partition_index: i32,
}

impl ApiResponse for ControlledShutdownResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let remaining_partitions = IndexSet::<RemainingPartition>::deserialize(version, bytes);
        (
            header,
            ControlledShutdownResponse {
                error_code,
                remaining_partitions,
            },
        )
    }
}

impl FromBytes for RemainingPartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        RemainingPartition {
            topic_name,
            partition_index,
        }
    }
}
