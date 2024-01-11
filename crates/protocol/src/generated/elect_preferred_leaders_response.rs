use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ElectPreferredLeadersResponse {
    pub throttle_time_ms: i32,
    pub replica_election_results: Vec<ReplicaElectionResult>,
}

#[derive(Debug, Default, Clone)]
pub struct ReplicaElectionResult {
    pub topic: String,
    pub partition_result: Vec<PartitionResult>,
}

#[derive(Debug, Default, Clone)]
pub struct PartitionResult {
    pub partition_id: i32,
    pub error_code: i16,
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
