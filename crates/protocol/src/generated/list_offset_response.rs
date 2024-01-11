use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ListOffsetResponse {
    pub throttle_time_ms: i32,
    pub topics: Vec<ListOffsetTopicResponse>,
}

#[derive(Debug, Default, Clone)]
pub struct ListOffsetTopicResponse {
    pub name: String,
    pub partitions: Vec<ListOffsetPartitionResponse>,
}

#[derive(Debug, Default, Clone)]
pub struct ListOffsetPartitionResponse {
    pub partition_index: i32,
    pub error_code: i16,
    pub old_style_offsets: Vec<i64>,
    pub timestamp: i64,
    pub offset: i64,
    pub leader_epoch: i32,
}

impl ApiResponse for ListOffsetResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 2 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<ListOffsetTopicResponse>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            ListOffsetResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl FromBytes for ListOffsetTopicResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<ListOffsetPartitionResponse>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ListOffsetTopicResponse { name, partitions }
    }
}

impl FromBytes for ListOffsetPartitionResponse {
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
        let old_style_offsets = if version >= 0 {
            Vec::<i64>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let timestamp = if version >= 1 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let offset = if version >= 1 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = if version >= 4 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ListOffsetPartitionResponse {
            partition_index,
            error_code,
            old_style_offsets,
            timestamp,
            offset,
            leader_epoch,
        }
    }
}
