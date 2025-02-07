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

    fn get_api_key() -> ApiKey {
        ApiKey(9)
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
        if version >= ApiVersion(3) {
            self.throttle_time_ms.serialize(version, _bytes);
        }
        self.topics.serialize(version, _bytes);
        if version >= ApiVersion(2) {
            self.error_code.serialize(version, _bytes);
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(3) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<OffsetFetchResponseTopic>::deserialize(version, bytes);
        let error_code = if version >= ApiVersion(2) {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        OffsetFetchResponse {
            throttle_time_ms,
            topics,
            error_code,
        }
    }
}

impl OffsetFetchResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.validate_fields(_version)?;
        }
        if self.error_code != 0 && _version.0 < 2 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "error_code",
                *_version,
                "OffsetFetchResponse",
            ));
        }
        Ok(())
    }
}

impl ToBytes for OffsetFetchResponseTopic {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.partitions.serialize(version, _bytes);
    }
}

impl OffsetFetchResponseTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.partitions.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for OffsetFetchResponseTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<OffsetFetchResponsePartition>::deserialize(version, bytes);
        OffsetFetchResponseTopic { name, partitions }
    }
}

impl ToBytes for OffsetFetchResponsePartition {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.partition_index.serialize(version, _bytes);
        self.committed_offset.serialize(version, _bytes);
        if version >= ApiVersion(5) {
            self.committed_leader_epoch.serialize(version, _bytes);
        }
        self.metadata.serialize(version, _bytes);
        self.error_code.serialize(version, _bytes);
    }
}

impl OffsetFetchResponsePartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.committed_leader_epoch != i32::default() && _version.0 < 5 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "committed_leader_epoch",
                *_version,
                "OffsetFetchResponsePartition",
            ));
        }
        Ok(())
    }
}

impl FromBytes for OffsetFetchResponsePartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let committed_offset = i64::deserialize(version, bytes);
        let committed_leader_epoch = if version >= ApiVersion(5) {
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
