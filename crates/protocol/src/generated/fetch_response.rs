use super::super::prelude::*;

///
/// Version 1 adds throttle time.
///
/// Version 2 and 3 are the same as version 1.
///
/// Version 4 adds features for transactional consumption.
///
/// Version 5 adds LogStartOffset to indicate the earliest available offset of
/// partition data that can be consumed.
///
/// Starting in version 6, we may return KAFKA_STORAGE_ERROR as an error code.
///
/// Version 7 adds incremental fetch request support.
///
/// Starting in version 8, on quota violation, brokers send out responses before throttling.
///
/// Version 9 is the same as version 8.
///
/// Version 10 indicates that the response data can use the ZStd compression
/// algorithm, as described in KIP-110.
///
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct FetchableTopicResponse {
    /// The topic name.
    pub name: String,

    /// The topic partitions.
    pub partitions: Vec<FetchablePartitionResponse>,
}

#[derive(Clone, Debug, PartialEq)]
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
    pub aborted: Option<Vec<AbortedTransaction>>,

    /// The record data.
    pub records: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Default)]
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
        let topics = Vec::<FetchableTopicResponse>::deserialize(version, bytes);
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
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<FetchablePartitionResponse>::deserialize(version, bytes);
        FetchableTopicResponse { name, partitions }
    }
}

impl FromBytes for FetchablePartitionResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let high_watermark = i64::deserialize(version, bytes);
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
            Option::<Vec<AbortedTransaction>>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let records = Option::<Vec<u8>>::deserialize(version, bytes);
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
