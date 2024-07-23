use super::super::prelude::*;

/// Starting in version 1, on quota violation brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlterReplicaLogDirsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub results: Vec<AlterReplicaLogDirTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterReplicaLogDirTopicResult {
    /// The name of the topic.
    pub topic_name: String,

    /// The results for each partition.
    pub partitions: Vec<AlterReplicaLogDirPartitionResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterReplicaLogDirPartitionResult {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for AlterReplicaLogDirsResponse {
    type Request = super::alter_replica_log_dirs_request::AlterReplicaLogDirsRequest;

    fn get_api_key() -> i16 {
        34
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
        let results = Vec::<AlterReplicaLogDirTopicResult>::deserialize(version, bytes);
        (
            header,
            AlterReplicaLogDirsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl AlterReplicaLogDirsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for AlterReplicaLogDirTopicResult {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic_name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirTopicResult {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterReplicaLogDirTopicResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partitions = Vec::<AlterReplicaLogDirPartitionResult>::deserialize(version, bytes);
        AlterReplicaLogDirTopicResult {
            topic_name,
            partitions,
        }
    }
}

impl ToBytes for AlterReplicaLogDirPartitionResult {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirPartitionResult {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterReplicaLogDirPartitionResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        AlterReplicaLogDirPartitionResult {
            partition_index,
            error_code,
        }
    }
}
