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

    fn get_api_key() -> ApiKey {
        ApiKey(1)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(10)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.replica_id.serialize(version, _bytes)?;
        self.max_wait.serialize(version, _bytes)?;
        self.min_bytes.serialize(version, _bytes)?;
        if version >= ApiVersion(3) {
            self.max_bytes.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(4) {
            self.isolation_level.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(7) {
            self.session_id.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(7) {
            self.epoch.serialize(version, _bytes)?;
        }
        self.topics.serialize(version, _bytes)?;
        if version >= ApiVersion(7) {
            self.forgotten.serialize(version, _bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let replica_id = i32::deserialize(version, bytes);
        let max_wait = i32::deserialize(version, bytes);
        let min_bytes = i32::deserialize(version, bytes);
        let max_bytes = if version >= ApiVersion(3) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let isolation_level = if version >= ApiVersion(4) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let session_id = if version >= ApiVersion(7) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let epoch = if version >= ApiVersion(7) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<FetchableTopic>::deserialize(version, bytes);
        let forgotten = if version >= ApiVersion(7) {
            Vec::<ForgottenTopic>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        FetchRequest {
            replica_id,
            max_wait,
            min_bytes,
            max_bytes,
            isolation_level,
            session_id,
            epoch,
            topics,
            forgotten,
        }
    }
}

impl FetchRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.isolation_level != i8::default() && _version >= ApiVersion(4) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "isolation_level",
                *_version,
                "FetchRequest",
            ));
        }
        if self.session_id != i32::default() && _version >= ApiVersion(7) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "session_id",
                *_version,
                "FetchRequest",
            ));
        }
        if self.epoch != i32::default() && _version >= ApiVersion(7) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "epoch",
                *_version,
                "FetchRequest",
            ));
        }
        if self.forgotten != Vec::<ForgottenTopic>::default() && _version >= ApiVersion(7) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "forgotten",
                *_version,
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
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        self.fetch_partitions.serialize(version, _bytes)?;
        Ok(())
    }
}

impl FetchableTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for FetchableTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let fetch_partitions = Vec::<FetchPartition>::deserialize(version, bytes);
        FetchableTopic {
            name,
            fetch_partitions,
        }
    }
}

impl ToBytes for ForgottenTopic {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= ApiVersion(7) {
            self.name.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(7) {
            self.forgotten_partition_indexes
                .serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl ForgottenTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.name != String::default() && _version >= ApiVersion(7) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                *_version,
                "ForgottenTopic",
            ));
        }
        if self.forgotten_partition_indexes != Vec::<i32>::default() && _version >= ApiVersion(7) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "forgotten_partition_indexes",
                *_version,
                "ForgottenTopic",
            ));
        }
        Ok(())
    }
}

impl FromBytes for ForgottenTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = if version >= ApiVersion(7) {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let forgotten_partition_indexes = if version >= ApiVersion(7) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ForgottenTopic {
            name,
            forgotten_partition_indexes,
        }
    }
}

impl ToBytes for FetchPartition {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, _bytes)?;
        if version >= ApiVersion(9) {
            self.current_leader_epoch.serialize(version, _bytes)?;
        }
        self.fetch_offset.serialize(version, _bytes)?;
        if version >= ApiVersion(5) {
            self.log_start_offset.serialize(version, _bytes)?;
        }
        self.max_bytes.serialize(version, _bytes)?;
        Ok(())
    }
}

impl FetchPartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.log_start_offset != i64::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "log_start_offset",
                *_version,
                "FetchPartition",
            ));
        }
        Ok(())
    }
}

impl FromBytes for FetchPartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let current_leader_epoch = if version >= ApiVersion(9) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let fetch_offset = i64::deserialize(version, bytes);
        let log_start_offset = if version >= ApiVersion(5) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let max_bytes = i32::deserialize(version, bytes);
        FetchPartition {
            partition_index,
            current_leader_epoch,
            fetch_offset,
            log_start_offset,
            max_bytes,
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
