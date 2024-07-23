use super::super::prelude::*;

/// Versions 1 and 2 are the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ControlledShutdownResponse {
    /// The top-level error code.
    pub error_code: i16,

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

    fn get_api_key() -> i16 {
        7
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
        self.remaining_partitions.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let remaining_partitions = IndexSet::<RemainingPartition>::deserialize(version, bytes);
        (
            header,
            ControlledShutdownResponse {
                error_code,
                remaining_partitions,
            },
        )
    }
}

impl ControlledShutdownResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for RemainingPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic_name.serialize(version, bytes)?;
        self.partition_index.serialize(version, bytes)?;
        Ok(())
    }
}

impl RemainingPartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for RemainingPartition {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        RemainingPartition {
            topic_name,
            partition_index,
        }
    }
}
