use super::super::prelude::*;

/// Version 1 adds KAFKA_STORAGE_ERROR as a valid error code.
///
/// Version 2 is the same as version 1.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeaderAndIsrResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: Option<ApiError>,

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
    pub error_code: Option<ApiError>,
}

impl ApiResponse for LeaderAndIsrResponse {
    type Request = super::leader_and_isr_request::LeaderAndIsrRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(4)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(2)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes);
        self.partitions.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = Option::<ApiError>::deserialize(version, bytes);
        let partitions = Vec::<LeaderAndIsrResponsePartition>::deserialize(version, bytes);
        LeaderAndIsrResponse {
            error_code,
            partitions,
        }
    }
}

impl LeaderAndIsrResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.partitions.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for LeaderAndIsrResponsePartition {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.topic_name.serialize(version, _bytes);
        self.partition_index.serialize(version, _bytes);
        self.error_code.serialize(version, _bytes);
    }
}

impl LeaderAndIsrResponsePartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for LeaderAndIsrResponsePartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        let error_code = Option::<ApiError>::deserialize(version, bytes);
        LeaderAndIsrResponsePartition {
            topic_name,
            partition_index,
            error_code,
        }
    }
}
