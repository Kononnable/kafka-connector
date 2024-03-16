use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProduceResponse {
    /// Each produce response
    pub responses: Vec<TopicProduceResponse>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TopicProduceResponse {
    /// The topic name
    pub name: String,

    /// Each partition that we produced to within the topic.
    pub partitions: Vec<PartitionProduceResponse>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionProduceResponse {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The base offset.
    pub base_offset: i64,

    /// The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended.
    pub log_append_time_ms: i64,

    /// The log start offset.
    pub log_start_offset: i64,
}

impl ApiResponse for ProduceResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let responses = Vec::<TopicProduceResponse>::deserialize(version, bytes);
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            ProduceResponse {
                responses,
                throttle_time_ms,
            },
        )
    }
}

impl FromBytes for TopicProduceResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<PartitionProduceResponse>::deserialize(version, bytes);
        TopicProduceResponse { name, partitions }
    }
}

impl FromBytes for PartitionProduceResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let base_offset = i64::deserialize(version, bytes);
        let log_append_time_ms = if version >= 2 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let log_start_offset = if version >= 5 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        PartitionProduceResponse {
            partition_index,
            error_code,
            base_offset,
            log_append_time_ms,
            log_start_offset,
        }
    }
}

impl Default for PartitionProduceResponse {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            error_code: Default::default(),
            base_offset: Default::default(),
            log_append_time_ms: -1,
            log_start_offset: -1,
        }
    }
}
