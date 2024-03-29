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
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
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

impl FromBytes for OffsetCommitResponseTopic {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<OffsetCommitResponsePartition>::deserialize(version, bytes);
        OffsetCommitResponseTopic { name, partitions }
    }
}

impl FromBytes for OffsetCommitResponsePartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        OffsetCommitResponsePartition {
            partition_index,
            error_code,
        }
    }
}
