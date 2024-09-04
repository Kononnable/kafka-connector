use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteRecordsRequest {
    /// Each topic that we want to delete records from.
    pub topics: Vec<DeleteRecordsTopic>,

    /// How long to wait for the deletion to complete, in milliseconds.
    pub timeout_ms: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeleteRecordsTopic {
    /// The topic name.
    pub name: String,

    /// Each partition that we want to delete records from.
    pub partitions: Vec<DeleteRecordsPartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeleteRecordsPartition {
    /// The partition index.
    pub partition_index: i32,

    /// The deletion offset.
    pub offset: i64,
}

impl ApiRequest for DeleteRecordsRequest {
    type Response = super::delete_records_response::DeleteRecordsResponse;

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
        self.topics.serialize(version, _bytes)?;
        self.timeout_ms.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topics = Vec::<DeleteRecordsTopic>::deserialize(version, bytes);
        let timeout_ms = i32::deserialize(version, bytes);
        DeleteRecordsRequest { topics, timeout_ms }
    }
}

impl DeleteRecordsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DeleteRecordsTopic {
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

impl DeleteRecordsTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeleteRecordsTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<DeleteRecordsPartition>::deserialize(version, bytes);
        DeleteRecordsTopic { name, partitions }
    }
}

impl ToBytes for DeleteRecordsPartition {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, _bytes)?;
        self.offset.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DeleteRecordsPartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeleteRecordsPartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let offset = i64::deserialize(version, bytes);
        DeleteRecordsPartition {
            partition_index,
            offset,
        }
    }
}
