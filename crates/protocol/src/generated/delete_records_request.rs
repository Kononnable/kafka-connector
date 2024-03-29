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

    fn get_api_key() -> i16 {
        21
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
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.topics.serialize(version, bytes)?;
        self.timeout_ms.serialize(version, bytes)?;
        Ok(())
    }
}

impl DeleteRecordsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DeleteRecordsTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl DeleteRecordsTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DeleteRecordsPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.offset.serialize(version, bytes)?;
        Ok(())
    }
}

impl DeleteRecordsPartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
