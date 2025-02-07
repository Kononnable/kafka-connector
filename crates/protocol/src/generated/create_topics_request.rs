use super::super::prelude::*;

/// Version 1 adds validateOnly.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreateTopicsRequest {
    /// The topics to create.
    pub topics: Vec<CreatableTopic>,

    /// How long to wait in milliseconds before timing out the request.
    pub timeout_ms: i32,

    /// If true, check that the topics can be created as specified, but don't create anything.
    pub validate_only: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatableTopic {
    /// The topic name.
    pub name: String,

    /// The number of partitions to create in the topic, or -1 if we are specifying a manual partition assignment.
    pub num_partitions: i32,

    /// The number of replicas to create for each partition in the topic, or -1 if we are specifying a manual partition assignment.
    pub replication_factor: i16,

    /// The manual partition assignment, or the empty array if we are using automatic assignment.
    pub assignments: IndexMap<CreatableReplicaAssignmentKey, CreatableReplicaAssignment>,

    /// The custom topic configurations to set.
    pub configs: IndexMap<CreateableTopicConfigKey, CreateableTopicConfig>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct CreatableReplicaAssignmentKey {
    /// The partition index.
    pub partition_index: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatableReplicaAssignment {
    /// The brokers to place the partition on.
    pub broker_ids: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct CreateableTopicConfigKey {
    /// The configuration name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreateableTopicConfig {
    /// The configuration value.
    pub value: Option<String>,
}

impl ApiRequest for CreateTopicsRequest {
    type Response = super::create_topics_response::CreateTopicsResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(19)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(3)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.topics.serialize(version, _bytes);
        self.timeout_ms.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.validate_only.serialize(version, _bytes);
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topics = Vec::<CreatableTopic>::deserialize(version, bytes);
        let timeout_ms = i32::deserialize(version, bytes);
        let validate_only = if version >= ApiVersion(1) {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        CreateTopicsRequest {
            topics,
            timeout_ms,
            validate_only,
        }
    }
}

impl CreateTopicsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.validate_fields(_version)?;
        }
        if self.validate_only != false && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "validate_only",
                *_version,
                "CreateTopicsRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for CreatableTopic {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.num_partitions.serialize(version, _bytes);
        self.replication_factor.serialize(version, _bytes);
        self.assignments.serialize(version, _bytes);
        self.configs.serialize(version, _bytes);
    }
}

impl CreatableTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.assignments.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        for item in self.configs.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for CreatableTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let num_partitions = i32::deserialize(version, bytes);
        let replication_factor = i16::deserialize(version, bytes);
        let assignments =
            IndexMap::<CreatableReplicaAssignmentKey, CreatableReplicaAssignment>::deserialize(
                version, bytes,
            );
        let configs = IndexMap::<CreateableTopicConfigKey, CreateableTopicConfig>::deserialize(
            version, bytes,
        );
        CreatableTopic {
            name,
            num_partitions,
            replication_factor,
            assignments,
            configs,
        }
    }
}

impl ToBytes for CreatableReplicaAssignmentKey {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.partition_index.serialize(version, _bytes);
    }
}

impl CreatableReplicaAssignmentKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatableReplicaAssignmentKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        CreatableReplicaAssignmentKey { partition_index }
    }
}

impl ToBytes for CreatableReplicaAssignment {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.broker_ids.serialize(version, _bytes);
    }
}

impl CreatableReplicaAssignment {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatableReplicaAssignment {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let broker_ids = Vec::<i32>::deserialize(version, bytes);
        CreatableReplicaAssignment { broker_ids }
    }
}

impl ToBytes for CreateableTopicConfigKey {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
    }
}

impl CreateableTopicConfigKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreateableTopicConfigKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        CreateableTopicConfigKey { name }
    }
}

impl ToBytes for CreateableTopicConfig {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.value.serialize(version, _bytes);
    }
}

impl CreateableTopicConfig {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreateableTopicConfig {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let value = Option::<String>::deserialize(version, bytes);
        CreateableTopicConfig { value }
    }
}
