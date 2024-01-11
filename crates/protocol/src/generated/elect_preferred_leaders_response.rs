use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct ElectPreferredLeadersResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub replica_election_results: Vec<ReplicaElectionResult>,
}

#[derive(Debug, Clone)]
pub struct ReplicaElectionResult {
    /// The topic name
    pub topic: String,

    /// The results for each partition
    pub partition_result: Vec<PartitionResult>,
}

#[derive(Debug, Clone)]
pub struct PartitionResult {
    /// The partition id
    pub partition_id: i32,

    /// The result error, or zero if there was no error.
    pub error_code: i16,

    /// The result message, or null if there was no error.
    pub error_message: String,
}

impl ApiResponse for ElectPreferredLeadersResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let replica_election_results = if version >= 0 {
            Vec::<ReplicaElectionResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            ElectPreferredLeadersResponse {
                throttle_time_ms,
                replica_election_results,
            },
        )
    }
}

impl Default for ElectPreferredLeadersResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            replica_election_results: Default::default(),
        }
    }
}

impl FromBytes for ReplicaElectionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let topic = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_result = if version >= 0 {
            Vec::<PartitionResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ReplicaElectionResult {
            topic,
            partition_result,
        }
    }
}

impl Default for ReplicaElectionResult {
    fn default() -> Self {
        Self {
            topic: Default::default(),
            partition_result: Default::default(),
        }
    }
}

impl FromBytes for PartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_id = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_message = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        PartitionResult {
            partition_id,
            error_code,
            error_message,
        }
    }
}

impl Default for PartitionResult {
    fn default() -> Self {
        Self {
            partition_id: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
