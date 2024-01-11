use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct FetchResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub session_id: i32,
    pub topics: Vec<FetchableTopicResponse>,
}

#[derive(Debug, Default, Clone)]
pub struct FetchableTopicResponse {
    pub name: String,
    pub partitions: Vec<FetchablePartitionResponse>,
}

#[derive(Debug, Default, Clone)]
pub struct FetchablePartitionResponse {
    pub partition_index: i32,
    pub error_code: i16,
    pub high_watermark: i64,
    pub last_stable_offset: i64,
    pub log_start_offset: i64,
    pub aborted: Vec<AbortedTransaction>,
    pub records: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct AbortedTransaction {
    pub producer_id: i64,
    pub first_offset: i64,
}

impl ApiResponse for FetchResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 7 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let session_id = if version >= 7 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<FetchableTopicResponse>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            FetchResponse {
                throttle_time_ms,
                error_code,
                session_id,
                topics,
            },
        )
    }
}

impl FromBytes for FetchableTopicResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<FetchablePartitionResponse>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        FetchableTopicResponse { name, partitions }
    }
}

impl FromBytes for FetchablePartitionResponse {
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
        let high_watermark = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let last_stable_offset = if version >= 4 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let log_start_offset = if version >= 5 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let aborted = if version >= 4 {
            Vec::<AbortedTransaction>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let records = if version >= 0 {
            Vec::<u8>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        FetchablePartitionResponse {
            partition_index,
            error_code,
            high_watermark,
            last_stable_offset,
            log_start_offset,
            aborted,
            records,
        }
    }
}

impl FromBytes for AbortedTransaction {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let producer_id = if version >= 4 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let first_offset = if version >= 4 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        AbortedTransaction {
            producer_id,
            first_offset,
        }
    }
}
