use super::super::prelude::*;

/// Version 1 removes MaxNumOffsets.  From this version forward, only a single
/// offset can be returned.
/// Version 2 adds the isolation level, which is used for transactional reads.
/// Version 3 is the same as version 2.
/// Version 4 adds the current leader epoch, which is used for fencing.
/// Version 5 is the same as version 5.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListOffsetRequest {
    /// The broker ID of the requestor, or -1 if this request is being made by a normal consumer.
    pub replica_id: i32,

    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
    pub isolation_level: i8,

    /// Each topic in the request.
    pub topics: Vec<ListOffsetTopic>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ListOffsetTopic {
    /// The topic name.
    pub name: String,

    /// Each partition in the request.
    pub partitions: Vec<ListOffsetPartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
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

    fn get_api_key() -> ApiKey {
        ApiKey(2)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(5)
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
        if version >= ApiVersion(2) {
            self.isolation_level.serialize(version, _bytes)?;
        }
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let replica_id = i32::deserialize(version, bytes);
        let isolation_level = if version >= ApiVersion(2) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<ListOffsetTopic>::deserialize(version, bytes);
        ListOffsetRequest {
            replica_id,
            isolation_level,
            topics,
        }
    }
}

impl ListOffsetRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.isolation_level != i8::default() && _version >= ApiVersion(2) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "isolation_level",
                *_version,
                "ListOffsetRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for ListOffsetTopic {
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

impl ListOffsetTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for ListOffsetTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<ListOffsetPartition>::deserialize(version, bytes);
        ListOffsetTopic { name, partitions }
    }
}

impl ToBytes for ListOffsetPartition {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, _bytes)?;
        if version >= ApiVersion(4) {
            self.current_leader_epoch.serialize(version, _bytes)?;
        }
        self.timestamp.serialize(version, _bytes)?;
        if version >= ApiVersion(0) {
            self.max_num_offsets.serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl ListOffsetPartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.current_leader_epoch != i32::default() && _version >= ApiVersion(4) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "current_leader_epoch",
                *_version,
                "ListOffsetPartition",
            ));
        }
        if self.max_num_offsets != i32::default() && _version >= ApiVersion(0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "max_num_offsets",
                *_version,
                "ListOffsetPartition",
            ));
        }
        Ok(())
    }
}

impl FromBytes for ListOffsetPartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let current_leader_epoch = if version >= ApiVersion(4) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let timestamp = i64::deserialize(version, bytes);
        let max_num_offsets = if version >= ApiVersion(0) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ListOffsetPartition {
            partition_index,
            current_leader_epoch,
            timestamp,
            max_num_offsets,
        }
    }
}
