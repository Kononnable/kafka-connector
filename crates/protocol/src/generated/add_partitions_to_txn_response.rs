use super::super::prelude::*;

/// Starting in version 1, on quota violation brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AddPartitionsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub results: IndexMap<AddPartitionsToTxnTopicResultKey, AddPartitionsToTxnTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AddPartitionsToTxnTopicResultKey {
    /// The topic name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AddPartitionsToTxnTopicResult {
    /// The results for each partition
    pub results: IndexMap<AddPartitionsToTxnPartitionResultKey, AddPartitionsToTxnPartitionResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AddPartitionsToTxnPartitionResultKey {
    /// The partition indexes.
    pub partition_index: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AddPartitionsToTxnPartitionResult {
    /// The response error code.
    pub error_code: i16,
}

impl ApiResponse for AddPartitionsToTxnResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = IndexMap::<AddPartitionsToTxnTopicResultKey,AddPartitionsToTxnTopicResult>::deserialize(version, bytes)
;
        (
            header,
            AddPartitionsToTxnResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for AddPartitionsToTxnTopicResultKey {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        AddPartitionsToTxnTopicResultKey { name }
    }
}

impl FromBytes for AddPartitionsToTxnTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let results = IndexMap::<
            AddPartitionsToTxnPartitionResultKey,
            AddPartitionsToTxnPartitionResult,
        >::deserialize(version, bytes);
        AddPartitionsToTxnTopicResult { results }
    }
}

impl FromBytes for AddPartitionsToTxnPartitionResultKey {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        AddPartitionsToTxnPartitionResultKey { partition_index }
    }
}

impl FromBytes for AddPartitionsToTxnPartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = i16::deserialize(version, bytes);
        AddPartitionsToTxnPartitionResult { error_code }
    }
}
