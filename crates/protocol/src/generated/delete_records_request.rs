use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DeleteRecordsRequest {
    /// Each topic that we want to delete records from.
    pub topics: Vec<DeleteRecordsTopic>,

    /// How long to wait for the deletion to complete, in milliseconds.
    pub timeout_ms: i32,
}

#[derive(Debug, Clone)]
pub struct DeleteRecordsTopic {
    /// The topic name.
    pub name: String,

    /// Each partition that we want to delete records from.
    pub partitions: Vec<DeleteRecordsPartition>,
}

#[derive(Debug, Clone)]
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
        if version >= 0 {
            self.timeout_ms.serialize(version, bytes);
        }
    }
}

impl Default for DeleteRecordsRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            timeout_ms: Default::default(),
        }
    }
}

impl ToBytes for DeleteRecordsTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}

impl Default for DeleteRecordsTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl ToBytes for DeleteRecordsPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 0 {
            self.offset.serialize(version, bytes);
        }
    }
}

impl Default for DeleteRecordsPartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            offset: Default::default(),
        }
    }
}
