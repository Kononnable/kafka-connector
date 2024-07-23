use super::super::prelude::*;

/// Version 1 adds KAFKA_STORAGE_ERROR as a valid error code.
///
/// Version 2 is the same as version 1.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeaderAndIsrResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// Each partition.
    pub partitions: Vec<LeaderAndIsrResponsePartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LeaderAndIsrResponsePartition {
    /// The topic name.
    pub topic_name: String,

    /// The partition index.
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for LeaderAndIsrResponse {
    type Request = super::leader_and_isr_request::LeaderAndIsrRequest;

    fn get_api_key() -> i16 {
        4
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
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
        let partitions = Vec::<LeaderAndIsrResponsePartition>::deserialize(version, bytes);
        (
            header,
            LeaderAndIsrResponse {
                error_code,
                partitions,
            },
        )
    }
}

impl LeaderAndIsrResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for LeaderAndIsrResponsePartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic_name.serialize(version, bytes)?;
        self.partition_index.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        Ok(())
    }
}

impl LeaderAndIsrResponsePartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for LeaderAndIsrResponsePartition {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        LeaderAndIsrResponsePartition {
            topic_name,
            partition_index,
            error_code,
        }
    }
}
