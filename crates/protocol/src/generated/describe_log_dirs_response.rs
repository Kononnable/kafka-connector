use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeLogDirsResponse {
    pub throttle_time_ms: i32,
    pub results: Vec<DescribeLogDirsResult>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribeLogDirsResult {
    pub error_code: i16,
    pub log_dir: String,
    pub topics: Vec<DescribeLogDirsTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribeLogDirsTopic {
    pub name: String,
    pub partitions: Vec<DescribeLogDirsPartition>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribeLogDirsPartition {
    pub partition_index: i32,
    pub partition_size: i64,
    pub offset_lag: i64,
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
