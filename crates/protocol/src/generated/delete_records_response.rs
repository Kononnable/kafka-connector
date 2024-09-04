use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteRecordsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each topic that we wanted to delete records from.
    pub topics: Vec<DeleteRecordsTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeleteRecordsTopicResult {
    /// The topic name.
    pub name: String,

    /// Each partition that we wanted to delete records from.
    pub partitions: Vec<DeleteRecordsPartitionResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeleteRecordsPartitionResult {
    /// The partition index.
    pub partition_index: i32,

    /// The partition low water mark.
    pub low_watermark: i64,

    /// The deletion error code, or 0 if the deletion succeeded.
    pub error_code: i16,
}

impl ApiResponse for DeleteRecordsResponse {
    type Request = super::delete_records_request::DeleteRecordsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(21)
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
        self.throttle_time_ms.serialize(version, _bytes)?;
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let topics = Vec::<DeleteRecordsTopicResult>::deserialize(version, bytes);
        DeleteRecordsResponse {
            throttle_time_ms,
            topics,
        }
    }
}

impl DeleteRecordsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DeleteRecordsTopicResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        self.partitions.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DeleteRecordsTopicResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeleteRecordsTopicResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<DeleteRecordsPartitionResult>::deserialize(version, bytes);
        DeleteRecordsTopicResult { name, partitions }
    }
}

impl ToBytes for DeleteRecordsPartitionResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, _bytes)?;
        self.low_watermark.serialize(version, _bytes)?;
        self.error_code.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DeleteRecordsPartitionResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeleteRecordsPartitionResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let low_watermark = i64::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        DeleteRecordsPartitionResult {
            partition_index,
            low_watermark,
            error_code,
        }
    }
}
