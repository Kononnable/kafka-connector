use super::super::prelude::*;

/// Version 1 is the same as version 0.
///
/// Version 2 adds a top-level error code.
///
/// Version 3 adds the throttle time.
///
/// Starting in version 4, on quota violation, brokers send out responses before throttling.
///
/// Version 5 adds the leader epoch to the committed offset.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OffsetFetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses per topic.
    pub topics: Vec<OffsetFetchResponseTopic>,

    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OffsetFetchResponseTopic {
    /// The topic name.
    pub name: String,

    /// The responses per partition
    pub partitions: Vec<OffsetFetchResponsePartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OffsetFetchResponsePartition {
    /// The partition index.
    pub partition_index: i32,

    /// The committed message offset.
    pub committed_offset: i64,

    /// The leader epoch.
    pub committed_leader_epoch: i32,

    /// The partition metadata.
    pub metadata: Option<String>,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for OffsetFetchResponse {
    type Request = super::offset_fetch_request::OffsetFetchRequest;

    fn get_api_key() -> i16 {
        9
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
        if version >= 3 {
            self.throttle_time_ms.serialize(version, bytes)?;
        }
        self.topics.serialize(version, bytes)?;
        if version >= 2 {
            self.error_code.serialize(version, bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 3 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<OffsetFetchResponseTopic>::deserialize(version, bytes);
        let error_code = if version >= 2 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            OffsetFetchResponse {
                throttle_time_ms,
                topics,
                error_code,
            },
        )
    }
}

impl OffsetFetchResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.error_code != i16::default() && _version >= 2 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "error_code",
                _version,
                "OffsetFetchResponse",
            ));
        }
        Ok(())
    }
}

impl ToBytes for OffsetFetchResponseTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetFetchResponseTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for OffsetFetchResponseTopic {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<OffsetFetchResponsePartition>::deserialize(version, bytes);
        OffsetFetchResponseTopic { name, partitions }
    }
}

impl ToBytes for OffsetFetchResponsePartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.committed_offset.serialize(version, bytes)?;
        if version >= 5 {
            self.committed_leader_epoch.serialize(version, bytes)?;
        }
        self.metadata.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetFetchResponsePartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.metadata.is_none() {
            return Err(SerializationError::NullValue(
                "metadata",
                _version,
                "OffsetFetchResponsePartition",
            ));
        }
        if self.committed_leader_epoch != i32::default() && _version >= 5 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "committed_leader_epoch",
                _version,
                "OffsetFetchResponsePartition",
            ));
        }
        Ok(())
    }
}

impl FromBytes for OffsetFetchResponsePartition {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let committed_offset = i64::deserialize(version, bytes);
        let committed_leader_epoch = if version >= 5 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let metadata = Option::<String>::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        OffsetFetchResponsePartition {
            partition_index,
            committed_offset,
            committed_leader_epoch,
            metadata,
            error_code,
        }
    }
}
