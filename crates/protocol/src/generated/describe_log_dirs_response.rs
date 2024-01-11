use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DescribeLogDirsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The log directories.
    pub results: Vec<DescribeLogDirsResult>,
}

#[derive(Debug, Clone)]
pub struct DescribeLogDirsResult {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The absolute log directory path.
    pub log_dir: String,

    /// Each topic.
    pub topics: Vec<DescribeLogDirsTopic>,
}

#[derive(Debug, Clone)]
pub struct DescribeLogDirsTopic {
    /// The topic name.
    pub name: String,

    pub partitions: Vec<DescribeLogDirsPartition>,
}

#[derive(Debug, Clone)]
pub struct DescribeLogDirsPartition {
    /// The partition index.
    pub partition_index: i32,

    /// The size of the log segments in this partition in bytes.
    pub partition_size: i64,

    /// The lag of the log's LEO w.r.t. partition's HW (if it is the current log for the partition) or current replica's LEO (if it is the future log for the partition)
    pub offset_lag: i64,

    /// True if this log is created by AlterReplicaLogDirsRequest and will replace the current log of the replica in the future.
    pub is_future_key: bool,
}

impl ApiResponse for DescribeLogDirsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<DescribeLogDirsResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            DescribeLogDirsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl Default for DescribeLogDirsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            results: Default::default(),
        }
    }
}

impl FromBytes for DescribeLogDirsResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let log_dir = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<DescribeLogDirsTopic>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribeLogDirsResult {
            error_code,
            log_dir,
            topics,
        }
    }
}

impl Default for DescribeLogDirsResult {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            log_dir: Default::default(),
            topics: Default::default(),
        }
    }
}

impl FromBytes for DescribeLogDirsTopic {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<DescribeLogDirsPartition>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribeLogDirsTopic { name, partitions }
    }
}

impl Default for DescribeLogDirsTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl FromBytes for DescribeLogDirsPartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_size = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let offset_lag = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let is_future_key = if version >= 0 {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribeLogDirsPartition {
            partition_index,
            partition_size,
            offset_lag,
            is_future_key,
        }
    }
}

impl Default for DescribeLogDirsPartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            partition_size: Default::default(),
            offset_lag: Default::default(),
            is_future_key: Default::default(),
        }
    }
}
