use super::super::prelude::*;

///
/// Version 1 is the same as version 0.
///
/// Starting in Version 2, the requestor must be able to handle Kafka Log
/// Message format version 1.
///
/// Version 3 adds MaxBytes.  Starting in version 3, the partition ordering in
/// the request is now relevant.  Partitions will be processed in the order
/// they appear in the request.
///
/// Version 4 adds IsolationLevel.  Starting in version 4, the reqestor must be
/// able to handle Kafka log message format version 2.
///
/// Version 5 adds LogStartOffset to indicate the earliest available offset of
/// partition data that can be consumed.
///
/// Version 6 is the same as version 5.
///
/// Version 7 adds incremental fetch request support.
///
/// Version 8 is the same as version 7.
///
/// Version 9 adds CurrentLeaderEpoch, as described in KIP-320.
///
/// Version 10 indicates that we can use the ZStd compression algorithm, as
/// described in KIP-110.
///
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct FetchableTopic {
    /// The name of the topic to fetch.
    pub name: String,

    /// The partitions to fetch.
    pub fetch_partitions: Vec<FetchPartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ForgottenTopic {
    /// The partition name.
    pub name: String,

    /// The partitions indexes to forget.
    pub forgotten_partition_indexes: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq)]
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
        self.replica_id.serialize(version, bytes)?;
        self.max_wait.serialize(version, bytes)?;
        self.min_bytes.serialize(version, bytes)?;
        if version >= 3 {
            self.max_bytes.serialize(version, bytes)?;
        }
        if version >= 4 {
            self.isolation_level.serialize(version, bytes)?;
        }
        if version >= 7 {
            self.session_id.serialize(version, bytes)?;
        }
        if version >= 7 {
            self.epoch.serialize(version, bytes)?;
        }
        self.topics.serialize(version, bytes)?;
        if version >= 7 {
            self.forgotten.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl FetchRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.replica_id != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "replica_id",
                _version,
                "FetchRequest",
            ));
        }
        if self.max_wait != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "max_wait",
                _version,
                "FetchRequest",
            ));
        }
        if self.min_bytes != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "min_bytes",
                _version,
                "FetchRequest",
            ));
        }
        if self.isolation_level != i8::default() && _version >= 4 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "isolation_level",
                _version,
                "FetchRequest",
            ));
        }
        if self.session_id != i32::default() && _version >= 7 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "session_id",
                _version,
                "FetchRequest",
            ));
        }
        if self.epoch != i32::default() && _version >= 7 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "epoch",
                _version,
                "FetchRequest",
            ));
        }
        if self.topics != Vec::<FetchableTopic>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topics",
                _version,
                "FetchRequest",
            ));
        }
        if self.forgotten != Vec::<ForgottenTopic>::default() && _version >= 7 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "forgotten",
                _version,
                "FetchRequest",
            ));
        }
        Ok(())
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.fetch_partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl FetchableTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "FetchableTopic",
            ));
        }
        if self.fetch_partitions != Vec::<FetchPartition>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "fetch_partitions",
                _version,
                "FetchableTopic",
            ));
        }
        Ok(())
    }
}

impl ToBytes for ForgottenTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 7 {
            self.name.serialize(version, bytes)?;
        }
        if version >= 7 {
            self.forgotten_partition_indexes.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl ForgottenTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() && _version >= 7 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "ForgottenTopic",
            ));
        }
        if self.forgotten_partition_indexes != Vec::<i32>::default() && _version >= 7 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "forgotten_partition_indexes",
                _version,
                "ForgottenTopic",
            ));
        }
        Ok(())
    }
}

impl ToBytes for FetchPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        if version >= 9 {
            self.current_leader_epoch.serialize(version, bytes)?;
        }
        self.fetch_offset.serialize(version, bytes)?;
        if version >= 5 {
            self.log_start_offset.serialize(version, bytes)?;
        }
        self.max_bytes.serialize(version, bytes)?;
        Ok(())
    }
}

impl FetchPartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.partition_index != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_index",
                _version,
                "FetchPartition",
            ));
        }
        if self.fetch_offset != i64::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "fetch_offset",
                _version,
                "FetchPartition",
            ));
        }
        if self.log_start_offset != i64::default() && _version >= 5 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "log_start_offset",
                _version,
                "FetchPartition",
            ));
        }
        if self.max_bytes != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "max_bytes",
                _version,
                "FetchPartition",
            ));
        }
        Ok(())
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
