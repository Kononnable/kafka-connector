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

    fn get_api_key() -> i16 {
        2
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        5
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
        if version >= 2 {
            self.throttle_time_ms.serialize(version, bytes)?;
        }
        self.topics.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 2 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<ListOffsetTopicResponse>::deserialize(version, bytes);
        (
            header,
            ListOffsetResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl ListOffsetResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for ListOffsetTopicResponse {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl ListOffsetTopicResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for ListOffsetTopicResponse {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<ListOffsetPartitionResponse>::deserialize(version, bytes);
        ListOffsetTopicResponse { name, partitions }
    }
}

impl ToBytes for ListOffsetPartitionResponse {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        if version >= 0 {
            self.old_style_offsets.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.timestamp.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.offset.serialize(version, bytes)?;
        }
        if version >= 4 {
            self.leader_epoch.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl ListOffsetPartitionResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.old_style_offsets != Vec::<i64>::default() && _version >= 0 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "old_style_offsets",
                _version,
                "ListOffsetPartitionResponse",
            ));
        }
        if self.timestamp != i64::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "timestamp",
                _version,
                "ListOffsetPartitionResponse",
            ));
        }
        if self.offset != i64::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "offset",
                _version,
                "ListOffsetPartitionResponse",
            ));
        }
        if self.leader_epoch != i32::default() && _version >= 4 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "leader_epoch",
                _version,
                "ListOffsetPartitionResponse",
            ));
        }
        Ok(())
    }
}

impl FromBytes for ListOffsetPartitionResponse {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let old_style_offsets = if version >= 0 {
            Vec::<i64>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let timestamp = if version >= 1 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let offset = if version >= 1 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = if version >= 4 {
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
