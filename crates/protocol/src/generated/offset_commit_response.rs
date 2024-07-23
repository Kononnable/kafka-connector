use super::super::prelude::*;

/// Versions 1 and 2 are the same as version 0.
///
/// Version 3 adds the throttle time to the response.
///
/// Starting in version 4, on quota violation, brokers send out responses before throttling.
///
/// Versions 5 and 6 are the same as version 4.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OffsetCommitResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses for each topic.
    pub topics: Vec<OffsetCommitResponseTopic>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OffsetCommitResponseTopic {
    /// The topic name.
    pub name: String,

    /// The responses for each partition in the topic.
    pub partitions: Vec<OffsetCommitResponsePartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OffsetCommitResponsePartition {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for OffsetCommitResponse {
    type Request = super::offset_commit_request::OffsetCommitRequest;

    fn get_api_key() -> i16 {
        8
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        6
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
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 3 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<OffsetCommitResponseTopic>::deserialize(version, bytes);
        (
            header,
            OffsetCommitResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl OffsetCommitResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for OffsetCommitResponseTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetCommitResponseTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for OffsetCommitResponseTopic {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<OffsetCommitResponsePartition>::deserialize(version, bytes);
        OffsetCommitResponseTopic { name, partitions }
    }
}

impl ToBytes for OffsetCommitResponsePartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetCommitResponsePartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for OffsetCommitResponsePartition {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        OffsetCommitResponsePartition {
            partition_index,
            error_code,
        }
    }
}
