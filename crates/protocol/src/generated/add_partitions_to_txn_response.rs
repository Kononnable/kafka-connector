use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct AddPartitionsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub results: Vec<AddPartitionsToTxnTopicResult>,
}

#[derive(Debug, Clone)]
pub struct AddPartitionsToTxnTopicResult {
    /// The topic name.
    pub name: String,

    /// The results for each partition
    pub results: Vec<AddPartitionsToTxnPartitionResult>,
}

#[derive(Debug, Clone)]
pub struct AddPartitionsToTxnPartitionResult {
    /// The partition indexes.
    pub partition_index: i32,

    /// The response error code.
    pub error_code: i16,
}

impl ApiResponse for AddPartitionsToTxnResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<AddPartitionsToTxnTopicResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            AddPartitionsToTxnResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl Default for AddPartitionsToTxnResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            results: Default::default(),
        }
    }
}

impl FromBytes for AddPartitionsToTxnTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<AddPartitionsToTxnPartitionResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        AddPartitionsToTxnTopicResult { name, results }
    }
}

impl Default for AddPartitionsToTxnTopicResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            results: Default::default(),
        }
    }
}

impl FromBytes for AddPartitionsToTxnPartitionResult {
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
        AddPartitionsToTxnPartitionResult {
            partition_index,
            error_code,
        }
    }
}

impl Default for AddPartitionsToTxnPartitionResult {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            error_code: Default::default(),
        }
    }
}
