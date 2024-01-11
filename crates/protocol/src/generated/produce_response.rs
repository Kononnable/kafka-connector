use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ProduceResponse {
    pub responses: Vec<TopicProduceResponse>,
    pub throttle_time_ms: i32,
}

#[derive(Debug, Default, Clone)]
pub struct TopicProduceResponse {
    pub name: String,
    pub partitions: Vec<PartitionProduceResponse>,
}

#[derive(Debug, Default, Clone)]
pub struct PartitionProduceResponse {
    pub partition_index: i32,
    pub error_code: i16,
    pub base_offset: i64,
    pub log_append_time_ms: i64,
    pub log_start_offset: i64,
}

impl ApiResponse for ProduceResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let responses = if version >= 0 {
            Vec::<TopicProduceResponse>::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<PartitionProduceResponse>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        TopicProduceResponse { name, partitions }
    }
}

impl FromBytes for PartitionProduceResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let base_offset = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
