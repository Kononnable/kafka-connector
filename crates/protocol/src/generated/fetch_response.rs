use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct FetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The top level response error code.
    pub error_code: i16,

    /// The fetch session ID, or 0 if this is not part of a fetch session.
    pub session_id: i32,

    /// The response topics.
    pub topics: Vec<FetchableTopicResponse>,
}

#[derive(Debug, Clone)]
pub struct FetchableTopicResponse {
    /// The topic name.
    pub name: String,

    /// The topic partitions.
    pub partitions: Vec<FetchablePartitionResponse>,
}

#[derive(Debug, Clone)]
pub struct FetchablePartitionResponse {
    /// The partiiton index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no fetch error.
    pub error_code: i16,

    /// The current high water mark.
    pub high_watermark: i64,

    /// The last stable offset (or LSO) of the partition. This is the last offset such that the state of all transactional records prior to this offset have been decided (ABORTED or COMMITTED)
    pub last_stable_offset: i64,

    /// The current log start offset.
    pub log_start_offset: i64,

    /// The aborted transactions.
    pub aborted: Vec<AbortedTransaction>,

    /// The record data.
    pub records: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct AbortedTransaction {
    /// The producer id associated with the aborted transaction.
    pub producer_id: i64,

    /// The first offset in the aborted transaction.
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

impl Default for FetchResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            error_code: Default::default(),
            session_id: 0,
            topics: Default::default(),
        }
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

impl Default for FetchableTopicResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
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

impl Default for FetchablePartitionResponse {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            error_code: Default::default(),
            high_watermark: Default::default(),
            last_stable_offset: -1,
            log_start_offset: -1,
            aborted: Default::default(),
            records: Default::default(),
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

impl Default for AbortedTransaction {
    fn default() -> Self {
        Self {
            producer_id: Default::default(),
            first_offset: Default::default(),
        }
    }
}
