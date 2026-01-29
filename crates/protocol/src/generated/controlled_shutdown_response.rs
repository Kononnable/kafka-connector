use super::super::prelude::*;

/// Versions 1 and 2 are the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ControlledShutdownResponse {
    /// The top-level error code.
    pub error_code: Option<ApiError>,

    /// The partitions that the broker still leads.
    pub remaining_partitions: IndexSet<RemainingPartition>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct RemainingPartition {
    /// The name of the topic.
    pub topic_name: String,

    /// The index of the partition.
    pub partition_index: i32,
}

impl ApiResponse for ControlledShutdownResponse {
    type Request = super::controlled_shutdown_request::ControlledShutdownRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(7)
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
        self.remaining_partitions.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = Option::<ApiError>::deserialize(version, bytes);
        let remaining_partitions = IndexSet::<RemainingPartition>::deserialize(version, bytes);
        ControlledShutdownResponse {
            error_code,
            remaining_partitions,
        }
    }
}

impl ControlledShutdownResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.remaining_partitions.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for RemainingPartition {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.topic_name.serialize(version, _bytes);
        self.partition_index.serialize(version, _bytes);
    }
}

impl RemainingPartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for RemainingPartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        RemainingPartition {
            topic_name,
            partition_index,
        }
    }
}
