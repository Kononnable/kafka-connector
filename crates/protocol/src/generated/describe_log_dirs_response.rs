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
    pub error_code: Option<ApiError>,

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

    fn get_api_key() -> ApiKey {
        ApiKey(35)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(1)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.throttle_time_ms.serialize(version, _bytes);
        self.results.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<DescribeLogDirsResult>::deserialize(version, bytes);
        DescribeLogDirsResponse {
            throttle_time_ms,
            results,
        }
    }
}

impl DescribeLogDirsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.results.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for DescribeLogDirsResult {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.error_code.serialize(version, _bytes);
        self.log_dir.serialize(version, _bytes);
        self.topics.serialize(version, _bytes);
    }
}

impl DescribeLogDirsResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for DescribeLogDirsResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = Option::<ApiError>::deserialize(version, bytes);
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
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.partitions.serialize(version, _bytes);
    }
}

impl DescribeLogDirsTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.partitions.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for DescribeLogDirsTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<DescribeLogDirsPartition>::deserialize(version, bytes);
        DescribeLogDirsTopic { name, partitions }
    }
}

impl ToBytes for DescribeLogDirsPartition {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.partition_index.serialize(version, _bytes);
        self.partition_size.serialize(version, _bytes);
        self.offset_lag.serialize(version, _bytes);
        self.is_future_key.serialize(version, _bytes);
    }
}

impl DescribeLogDirsPartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribeLogDirsPartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
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
