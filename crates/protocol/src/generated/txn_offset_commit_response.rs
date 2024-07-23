use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
/// Version 2 is the same as version 1.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TxnOffsetCommitResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses for each topic.
    pub topics: Vec<TxnOffsetCommitResponseTopic>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TxnOffsetCommitResponseTopic {
    /// The topic name.
    pub name: String,

    /// The responses for each partition in the topic.
    pub partitions: Vec<TxnOffsetCommitResponsePartition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TxnOffsetCommitResponsePartition {
    /// The partitition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for TxnOffsetCommitResponse {
    type Request = super::txn_offset_commit_request::TxnOffsetCommitRequest;

    fn get_api_key() -> i16 {
        28
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
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
        self.throttle_time_ms.serialize(version, bytes)?;
        self.topics.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let topics = Vec::<TxnOffsetCommitResponseTopic>::deserialize(version, bytes);
        (
            header,
            TxnOffsetCommitResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl TxnOffsetCommitResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for TxnOffsetCommitResponseTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl TxnOffsetCommitResponseTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for TxnOffsetCommitResponseTopic {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<TxnOffsetCommitResponsePartition>::deserialize(version, bytes);
        TxnOffsetCommitResponseTopic { name, partitions }
    }
}

impl ToBytes for TxnOffsetCommitResponsePartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        Ok(())
    }
}

impl TxnOffsetCommitResponsePartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for TxnOffsetCommitResponsePartition {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        TxnOffsetCommitResponsePartition {
            partition_index,
            error_code,
        }
    }
}
