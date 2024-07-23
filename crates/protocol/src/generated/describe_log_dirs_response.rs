use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeLogDirsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The log directories.
    pub results: Vec<DescribeLogDirsResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribeLogDirsResult {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The absolute log directory path.
    pub log_dir: String,

    /// Each topic.
    pub topics: Vec<DescribeLogDirsTopic>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribeLogDirsTopic {
    /// The topic name.
    pub name: String,

    pub partitions: Vec<DescribeLogDirsPartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
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
    type Request = super::describe_log_dirs_request::DescribeLogDirsRequest;

    fn get_api_key() -> i16 {
        35
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &ResponseHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.throttle_time_ms.serialize(version, bytes)?;
        self.results.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<DescribeLogDirsResult>::deserialize(version, bytes);
        (
            header,
            DescribeLogDirsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl DescribeLogDirsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DescribeLogDirsResult {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, bytes)?;
        self.log_dir.serialize(version, bytes)?;
        self.topics.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribeLogDirsResult {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribeLogDirsResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let log_dir = String::deserialize(version, bytes);
        let topics = Vec::<DescribeLogDirsTopic>::deserialize(version, bytes);
        DescribeLogDirsResult {
            error_code,
            log_dir,
            topics,
        }
    }
}

impl ToBytes for DescribeLogDirsTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribeLogDirsTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribeLogDirsTopic {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<DescribeLogDirsPartition>::deserialize(version, bytes);
        DescribeLogDirsTopic { name, partitions }
    }
}

impl ToBytes for DescribeLogDirsPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.partition_size.serialize(version, bytes)?;
        self.offset_lag.serialize(version, bytes)?;
        self.is_future_key.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribeLogDirsPartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribeLogDirsPartition {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let partition_size = i64::deserialize(version, bytes);
        let offset_lag = i64::deserialize(version, bytes);
        let is_future_key = bool::deserialize(version, bytes);
        DescribeLogDirsPartition {
            partition_index,
            partition_size,
            offset_lag,
            is_future_key,
        }
    }
}
