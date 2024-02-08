use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AddPartitionsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub results: Vec<AddPartitionsToTxnTopicResult>,
}

#[derive(Clone, Debug, Default)]
pub struct AddPartitionsToTxnTopicResult {
    /// The topic name.
    pub name: String,

    /// The results for each partition
    pub results: Vec<AddPartitionsToTxnPartitionResult>,
}

#[derive(Clone, Debug, Default)]
pub struct AddPartitionsToTxnPartitionResult {
    /// The partition indexes.
    pub partition_index: i32,

    /// The response error code.
    pub error_code: i16,
}

impl ApiResponse for AddPartitionsToTxnResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<AddPartitionsToTxnTopicResult>::deserialize(version, bytes);
        (
            header,
            AddPartitionsToTxnResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for AddPartitionsToTxnTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        let results = Vec::<AddPartitionsToTxnPartitionResult>::deserialize(version, bytes);
        AddPartitionsToTxnTopicResult { name, results }
    }
}

impl FromBytes for AddPartitionsToTxnPartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        AddPartitionsToTxnPartitionResult {
            partition_index,
            error_code,
        }
    }
}
