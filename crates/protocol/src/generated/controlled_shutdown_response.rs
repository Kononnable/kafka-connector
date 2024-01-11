use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct ControlledShutdownResponse {
    /// The top-level error code.
    pub error_code: i16,

    /// The partitions that the broker still leads.
    pub remaining_partitions: Vec<RemainingPartition>,
}

#[derive(Debug, Clone)]
pub struct RemainingPartition {
    /// The name of the topic.
    pub topic_name: String,

    /// The index of the partition.
    pub partition_index: i32,
}

impl ApiResponse for ControlledShutdownResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let remaining_partitions = if version >= 0 {
            Vec::<RemainingPartition>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            ControlledShutdownResponse {
                error_code,
                remaining_partitions,
            },
        )
    }
}

impl Default for ControlledShutdownResponse {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            remaining_partitions: Default::default(),
        }
    }
}

impl FromBytes for RemainingPartition {
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
        RemainingPartition {
            topic_name,
            partition_index,
        }
    }
}

impl Default for RemainingPartition {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_index: Default::default(),
        }
    }
}
