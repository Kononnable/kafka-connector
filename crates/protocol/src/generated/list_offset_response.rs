use super::super::prelude::*;

/// Version 1 removes the offsets array in favor of returning a single offset.
/// Version 1 also adds the timestamp associated with the returned offset.
/// Version 2 adds the throttle time.
/// Starting in version 3, on quota violation, brokers send out responses before throttling.
/// Version 4 adds the leader epoch, which is used for fencing.
/// Version 5 adds a new error code, OFFSET_NOT_AVAILABLE.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListOffsetResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each topic in the response.
    pub topics: Vec<ListOffsetTopicResponse>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ListOffsetTopicResponse {
    /// The topic name
    pub name: String,

    /// Each partition in the response.
    pub partitions: Vec<ListOffsetPartitionResponse>,
}

#[derive(Clone, Debug, PartialEq)]
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
    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 2 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<ListOffsetTopicResponse>::deserialize(version, bytes);
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
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<ListOffsetPartitionResponse>::deserialize(version, bytes);
        ListOffsetTopicResponse { name, partitions }
    }
}

impl FromBytes for ListOffsetPartitionResponse {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
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
