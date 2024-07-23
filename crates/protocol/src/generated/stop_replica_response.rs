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

    fn get_api_key() -> i16 {
        5
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
        self.error_code.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let partitions = Vec::<StopReplicaResponsePartition>::deserialize(version, bytes);
        (
            header,
            StopReplicaResponse {
                error_code,
                partitions,
            },
        )
    }
}

impl StopReplicaResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for StopReplicaResponsePartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic_name.serialize(version, bytes)?;
        self.partition_index.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        Ok(())
    }
}

impl StopReplicaResponsePartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for StopReplicaResponsePartition {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
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
