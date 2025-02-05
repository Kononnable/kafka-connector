use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElectPreferredLeadersResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub replica_election_results: Vec<ReplicaElectionResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ReplicaElectionResult {
    /// The topic name
    pub topic: String,

    /// The results for each partition
    pub partition_result: Vec<PartitionResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct PartitionResult {
    /// The partition id
    pub partition_id: i32,

    /// The result error, or zero if there was no error.
    pub error_code: i16,

    /// The result message, or null if there was no error.
    pub error_message: Option<String>,
}

impl ApiResponse for ElectPreferredLeadersResponse {
    type Request = super::elect_preferred_leaders_request::ElectPreferredLeadersRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(43)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.throttle_time_ms.serialize(version, _bytes)?;
        self.replica_election_results.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let replica_election_results = Vec::<ReplicaElectionResult>::deserialize(version, bytes);
        ElectPreferredLeadersResponse {
            throttle_time_ms,
            replica_election_results,
        }
    }
}

impl ElectPreferredLeadersResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for ReplicaElectionResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic.serialize(version, _bytes)?;
        self.partition_result.serialize(version, _bytes)?;
        Ok(())
    }
}

impl ReplicaElectionResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for ReplicaElectionResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic = String::deserialize(version, bytes);
        let partition_result = Vec::<PartitionResult>::deserialize(version, bytes);
        ReplicaElectionResult {
            topic,
            partition_result,
        }
    }
}

impl ToBytes for PartitionResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_id.serialize(version, _bytes)?;
        self.error_code.serialize(version, _bytes)?;
        self.error_message.serialize(version, _bytes)?;
        Ok(())
    }
}

impl PartitionResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for PartitionResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
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
