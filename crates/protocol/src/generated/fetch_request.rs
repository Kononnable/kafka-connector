use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct FetchRequest {
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    pub replica_id: i32,

    /// The maximum time in milliseconds to wait for the response.
    pub max_wait: i32,

    /// The minimum bytes to accumulate in the response.
    pub min_bytes: i32,

    /// The maximum bytes to fetch.  See KIP-74 for cases where this limit may not be honored.
    pub max_bytes: i32,

    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
    pub isolation_level: i8,

    /// The fetch session ID.
    pub session_id: i32,

    /// The fetch session ID.
    pub epoch: i32,

    /// The topics to fetch.
    pub topics: Vec<FetchableTopic>,

    /// In an incremental fetch request, the partitions to remove.
    pub forgotten: Vec<ForgottenTopic>,
}

#[derive(Debug, Clone)]
pub struct FetchableTopic {
    /// The name of the topic to fetch.
    pub name: String,

    /// The partitions to fetch.
    pub fetch_partitions: Vec<FetchPartition>,
}

#[derive(Debug, Clone)]
pub struct ForgottenTopic {
    /// The partition name.
    pub name: String,

    /// The partitions indexes to forget.
    pub forgotten_partition_indexes: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct FetchPartition {
    /// The partition index.
    pub partition_index: i32,

    /// The current leader epoch of the partition.
    pub current_leader_epoch: i32,

    /// The message offset.
    pub fetch_offset: i64,

    /// The earliest available offset of the follower replica.  The field is only used when the request is sent by the follower.
    pub log_start_offset: i64,

    /// The maximum bytes to fetch from this partition.  See KIP-74 for cases where this limit may not be honored.
    pub max_bytes: i32,
}

impl ApiRequest for FetchRequest {
    type Response = super::fetch_response::FetchResponse;

    fn get_api_key() -> i16 {
        1
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        10
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
        if version >= 0 {
            self.max_wait.serialize(version, bytes);
        }
        if version >= 0 {
            self.min_bytes.serialize(version, bytes);
        }
        if version >= 3 {
            self.max_bytes.serialize(version, bytes);
        }
        if version >= 4 {
            self.isolation_level.serialize(version, bytes);
        }
        if version >= 7 {
            self.session_id.serialize(version, bytes);
        }
        if version >= 7 {
            self.epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
        if version >= 7 {
            self.forgotten.serialize(version, bytes);
        }
    }
}

impl Default for FetchRequest {
    fn default() -> Self {
        Self {
            replica_id: Default::default(),
            max_wait: Default::default(),
            min_bytes: Default::default(),
            max_bytes: 0x7fffffff,
            isolation_level: 0,
            session_id: 0,
            epoch: -1,
            topics: Default::default(),
            forgotten: Default::default(),
        }
    }
}

impl ToBytes for FetchableTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.fetch_partitions.serialize(version, bytes);
        }
    }
}

impl Default for FetchableTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            fetch_partitions: Default::default(),
        }
    }
}

impl ToBytes for ForgottenTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 7 {
            self.name.serialize(version, bytes);
        }
        if version >= 7 {
            self.forgotten_partition_indexes.serialize(version, bytes);
        }
    }
}

impl Default for ForgottenTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            forgotten_partition_indexes: Default::default(),
        }
    }
}

impl ToBytes for FetchPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 9 {
            self.current_leader_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.fetch_offset.serialize(version, bytes);
        }
        if version >= 5 {
            self.log_start_offset.serialize(version, bytes);
        }
        if version >= 0 {
            self.max_bytes.serialize(version, bytes);
        }
    }
}

impl Default for FetchPartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            current_leader_epoch: -1,
            fetch_offset: Default::default(),
            log_start_offset: -1,
            max_bytes: Default::default(),
        }
    }
}
