use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct OffsetFetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses per topic.
    pub topics: Vec<OffsetFetchResponseTopic>,

    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
}

#[derive(Debug, Clone)]
pub struct OffsetFetchResponseTopic {
    /// The topic name.
    pub name: String,

    /// The responses per partition
    pub partitions: Vec<OffsetFetchResponsePartition>,
}

#[derive(Debug, Clone)]
pub struct OffsetFetchResponsePartition {
    /// The partition index.
    pub partition_index: i32,

    /// The committed message offset.
    pub committed_offset: i64,

    /// The leader epoch.
    pub committed_leader_epoch: i32,

    /// The partition metadata.
    pub metadata: String,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for OffsetFetchResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 3 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<OffsetFetchResponseTopic>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 2 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            OffsetFetchResponse {
                throttle_time_ms,
                topics,
                error_code,
            },
        )
    }
}

impl Default for OffsetFetchResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            topics: Default::default(),
            error_code: 0,
        }
    }
}

impl FromBytes for OffsetFetchResponseTopic {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<OffsetFetchResponsePartition>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        OffsetFetchResponseTopic { name, partitions }
    }
}

impl Default for OffsetFetchResponseTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl FromBytes for OffsetFetchResponsePartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let committed_offset = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let committed_leader_epoch = if version >= 5 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let metadata = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        OffsetFetchResponsePartition {
            partition_index,
            committed_offset,
            committed_leader_epoch,
            metadata,
            error_code,
        }
    }
}

impl Default for OffsetFetchResponsePartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            committed_offset: Default::default(),
            committed_leader_epoch: Default::default(),
            metadata: Default::default(),
            error_code: Default::default(),
        }
    }
}
