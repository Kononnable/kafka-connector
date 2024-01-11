use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct OffsetFetchResponse {
    pub throttle_time_ms: i32,
    pub topics: Vec<OffsetFetchResponseTopic>,
    pub error_code: i16,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetFetchResponseTopic {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponsePartition>,
}

#[derive(Debug, Default, Clone)]
pub struct OffsetFetchResponsePartition {
    pub partition_index: i32,
    pub committed_offset: i64,
    pub committed_leader_epoch: i32,
    pub metadata: String,
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
