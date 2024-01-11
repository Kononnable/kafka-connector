use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct ListOffsetRequest {
    /// The broker ID of the requestor, or -1 if this request is being made by a normal consumer.
    pub replica_id: i32,

    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
    pub isolation_level: i8,

    /// Each topic in the request.
    pub topics: Vec<ListOffsetTopic>,
}

#[derive(Debug, Clone)]
pub struct ListOffsetTopic {
    /// The topic name.
    pub name: String,

    /// Each partition in the request.
    pub partitions: Vec<ListOffsetPartition>,
}

#[derive(Debug, Clone)]
pub struct ListOffsetPartition {
    /// The partition index.
    pub partition_index: i32,

    /// The current leader epoch.
    pub current_leader_epoch: i32,

    /// The current timestamp.
    pub timestamp: i64,

    /// The maximum number of offsets to report.
    pub max_num_offsets: i32,
}

impl ApiRequest for ListOffsetRequest {
    type Response = super::list_offset_response::ListOffsetResponse;

    fn get_api_key() -> i16 {
        2
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        5
    }

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.replica_id.serialize(version, bytes);
        }
        if version >= 2 {
            self.isolation_level.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
    }
}

impl Default for ListOffsetRequest {
    fn default() -> Self {
        Self {
            replica_id: Default::default(),
            isolation_level: Default::default(),
            topics: Default::default(),
        }
    }
}

impl ToBytes for ListOffsetTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}

impl Default for ListOffsetTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl ToBytes for ListOffsetPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 4 {
            self.current_leader_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.timestamp.serialize(version, bytes);
        }
        if version >= 0 {
            self.max_num_offsets.serialize(version, bytes);
        }
    }
}

impl Default for ListOffsetPartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            current_leader_epoch: Default::default(),
            timestamp: Default::default(),
            max_num_offsets: Default::default(),
        }
    }
}
