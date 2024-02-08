use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ElectPreferredLeadersResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub replica_election_results: Vec<ReplicaElectionResult>,
}

#[derive(Clone, Debug, Default)]
pub struct ReplicaElectionResult {
    /// The topic name
    pub topic: String,

    /// The results for each partition
    pub partition_result: Vec<PartitionResult>,
}

#[derive(Clone, Debug, Default)]
pub struct PartitionResult {
    /// The partition id
    pub partition_id: i32,

    /// The result error, or zero if there was no error.
    pub error_code: i16,

    /// The result message, or null if there was no error.
    pub error_message: Option<String>,
}

impl ApiResponse for ElectPreferredLeadersResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let replica_election_results = Vec::<ReplicaElectionResult>::deserialize(version, bytes);
        (
            header,
            ElectPreferredLeadersResponse {
                throttle_time_ms,
                replica_election_results,
            },
        )
    }
}

impl FromBytes for ReplicaElectionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let topic = String::deserialize(version, bytes);
        let partition_result = Vec::<PartitionResult>::deserialize(version, bytes);
        ReplicaElectionResult {
            topic,
            partition_result,
        }
    }
}

impl FromBytes for PartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_id = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        PartitionResult {
            partition_id,
            error_code,
            error_message,
        }
    }
}
