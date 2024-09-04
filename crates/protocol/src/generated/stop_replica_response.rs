use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StopReplicaResponse {
    /// The top-level error code, or 0 if there was no top-level error.
    pub error_code: i16,

    /// The responses for each partition.
    pub partitions: Vec<StopReplicaResponsePartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StopReplicaResponsePartition {
    /// The topic name.
    pub topic_name: String,

    /// The partition index.
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no partition error.
    pub error_code: i16,
}

impl ApiResponse for StopReplicaResponse {
    type Request = super::stop_replica_request::StopReplicaRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(5)
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
        self.error_code.serialize(version, _bytes)?;
        self.partitions.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let partitions = Vec::<StopReplicaResponsePartition>::deserialize(version, bytes);
        StopReplicaResponse {
            error_code,
            partitions,
        }
    }
}

impl StopReplicaResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for StopReplicaResponsePartition {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic_name.serialize(version, _bytes)?;
        self.partition_index.serialize(version, _bytes)?;
        self.error_code.serialize(version, _bytes)?;
        Ok(())
    }
}

impl StopReplicaResponsePartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for StopReplicaResponsePartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        StopReplicaResponsePartition {
            topic_name,
            partition_index,
            error_code,
        }
    }
}
