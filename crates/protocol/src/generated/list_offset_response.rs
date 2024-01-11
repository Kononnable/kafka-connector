use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct ListOffsetResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each topic in the response.
    pub topics: Vec<ListOffsetTopicResponse>,
}

#[derive(Debug, Clone)]
pub struct ListOffsetTopicResponse {
    /// The topic name
    pub name: String,

    /// Each partition in the response.
    pub partitions: Vec<ListOffsetPartitionResponse>,
}

#[derive(Debug, Clone)]
pub struct ListOffsetPartitionResponse {
    /// The partition index.
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no error.
    pub error_code: i16,

    /// The result offsets.
    pub old_style_offsets: Vec<i64>,

    /// The timestamp associated with the returned offset.
    pub timestamp: i64,

    /// The returned offset.
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

impl Default for ListOffsetResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            topics: Default::default(),
        }
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

impl Default for ListOffsetTopicResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
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

impl Default for ListOffsetPartitionResponse {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            error_code: Default::default(),
            old_style_offsets: Default::default(),
            timestamp: -1,
            offset: -1,
            leader_epoch: Default::default(),
        }
    }
}
