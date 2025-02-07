use super::super::prelude::*;

/// Version 1 removes the offsets array in favor of returning a single offset.
/// Version 1 also adds the timestamp associated with the returned offset.
/// Version 2 adds the throttle time.
/// Starting in version 3, on quota violation, brokers send out responses before throttling.
/// Version 4 adds the leader epoch, which is used for fencing.
/// Version 5 adds a new error code, OFFSET_NOT_AVAILABLE.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListOffsetResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each topic in the response.
    pub topics: Vec<ListOffsetTopicResponse>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ListOffsetTopicResponse {
    /// The topic name
    pub name: String,

    /// Each partition in the response.
    pub partitions: Vec<ListOffsetPartitionResponse>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOffsetPartitionResponse {
    /// The partition index.
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no error.
    pub error_code: i16,

    /// The result offsets.
    pub old_style_offsets: Vec<i64>,

    /// The timestamp associated with the returned offset.
    pub timestamp: i64,

    /// The returned offset.
    pub offset: i64,

    pub leader_epoch: i32,
}

impl ApiResponse for ListOffsetResponse {
    type Request = super::list_offset_request::ListOffsetRequest;

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
        if version >= ApiVersion(2) {
            self.throttle_time_ms.serialize(version, _bytes);
        }
        self.topics.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(2) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<ListOffsetTopicResponse>::deserialize(version, bytes);
        ListOffsetResponse {
            throttle_time_ms,
            topics,
        }
    }
}

impl ListOffsetResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for ListOffsetTopicResponse {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.partitions.serialize(version, _bytes);
    }
}

impl ListOffsetTopicResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.partitions.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for ListOffsetTopicResponse {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<ListOffsetPartitionResponse>::deserialize(version, bytes);
        ListOffsetTopicResponse { name, partitions }
    }
}

impl ToBytes for ListOffsetPartitionResponse {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.partition_index.serialize(version, _bytes);
        self.error_code.serialize(version, _bytes);
        if version >= ApiVersion(0) {
            self.old_style_offsets.serialize(version, _bytes);
        }
        if version >= ApiVersion(1) {
            self.timestamp.serialize(version, _bytes);
        }
        if version >= ApiVersion(1) {
            self.offset.serialize(version, _bytes);
        }
        if version >= ApiVersion(4) {
            self.leader_epoch.serialize(version, _bytes);
        }
    }
}

impl ListOffsetPartitionResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.old_style_offsets != Vec::<i64>::default() && _version.0 < 0 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "old_style_offsets",
                *_version,
                "ListOffsetPartitionResponse",
            ));
        }
        if self.timestamp != -1 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "timestamp",
                *_version,
                "ListOffsetPartitionResponse",
            ));
        }
        if self.offset != -1 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "offset",
                *_version,
                "ListOffsetPartitionResponse",
            ));
        }
        if self.leader_epoch != i32::default() && _version.0 < 4 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "leader_epoch",
                *_version,
                "ListOffsetPartitionResponse",
            ));
        }
        Ok(())
    }
}

impl FromBytes for ListOffsetPartitionResponse {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let old_style_offsets = if version >= ApiVersion(0) {
            Vec::<i64>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let timestamp = if version >= ApiVersion(1) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let offset = if version >= ApiVersion(1) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = if version >= ApiVersion(4) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ListOffsetPartitionResponse {
            partition_index,
            error_code,
            old_style_offsets,
            timestamp,
            offset,
            leader_epoch,
        }
    }
}

impl Default for ListOffsetPartitionResponse {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            error_code: Default::default(),
            old_style_offsets: Default::default(),
            timestamp: -1,
            offset: -1,
            leader_epoch: Default::default(),
        }
    }
}
