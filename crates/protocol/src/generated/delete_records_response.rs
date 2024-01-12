use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteRecordsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each topic that we wanted to delete records from.
    pub topics: Vec<DeleteRecordsTopicResult>,
}

#[derive(Clone, Debug, Default)]
pub struct DeleteRecordsTopicResult {
    /// The topic name.
    pub name: String,

    /// Each partition that we wanted to delete records from.
    pub partitions: Vec<DeleteRecordsPartitionResult>,
}

#[derive(Clone, Debug, Default)]
pub struct DeleteRecordsPartitionResult {
    /// The partition index.
    pub partition_index: i32,

    /// The partition low water mark.
    pub low_watermark: i64,

    /// The deletion error code, or 0 if the deletion succeeded.
    pub error_code: i16,
}

impl ApiResponse for DeleteRecordsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<DeleteRecordsTopicResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            DeleteRecordsResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl FromBytes for DeleteRecordsTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<DeleteRecordsPartitionResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DeleteRecordsTopicResult { name, partitions }
    }
}

impl FromBytes for DeleteRecordsPartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let low_watermark = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DeleteRecordsPartitionResult {
            partition_index,
            low_watermark,
            error_code,
        }
    }
}
