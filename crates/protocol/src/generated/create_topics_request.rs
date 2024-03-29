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

    fn get_api_key() -> i16 {
        19
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        3
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.topics.serialize(version, bytes)?;
        self.timeout_ms.serialize(version, bytes)?;
        if version >= 1 {
            self.validate_only.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl CreateTopicsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topics != Vec::<CreatableTopic>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topics",
                _version,
                "CreateTopicsRequest",
            ));
        }
        if self.timeout_ms != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "timeout_ms",
                _version,
                "CreateTopicsRequest",
            ));
        }
        if self.validate_only != bool::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "validate_only",
                _version,
                "CreateTopicsRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for CreatableTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.num_partitions.serialize(version, bytes)?;
        self.replication_factor.serialize(version, bytes)?;
        self.assignments.serialize(version, bytes)?;
        self.configs.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreatableTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "CreatableTopic",
            ));
        }
        if self.num_partitions != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "num_partitions",
                _version,
                "CreatableTopic",
            ));
        }
        if self.replication_factor != i16::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "replication_factor",
                _version,
                "CreatableTopic",
            ));
        }
        if self.assignments
            != IndexMap::<CreatableReplicaAssignmentKey, CreatableReplicaAssignment>::default()
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "assignments",
                _version,
                "CreatableTopic",
            ));
        }
        if self.configs != IndexMap::<CreateableTopicConfigKey, CreateableTopicConfig>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "configs",
                _version,
                "CreatableTopic",
            ));
        }
        Ok(())
    }
}

impl ToBytes for CreatableReplicaAssignmentKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreatableReplicaAssignmentKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.partition_index != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_index",
                _version,
                "CreatableReplicaAssignmentKey",
            ));
        }
        Ok(())
    }
}

impl ToBytes for CreatableReplicaAssignment {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.broker_ids.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreatableReplicaAssignment {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.broker_ids != Vec::<i32>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "broker_ids",
                _version,
                "CreatableReplicaAssignment",
            ));
        }
        Ok(())
    }
}

impl ToBytes for CreateableTopicConfigKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreateableTopicConfigKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "CreateableTopicConfigKey",
            ));
        }
        Ok(())
    }
}

impl ToBytes for CreateableTopicConfig {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.value.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreateableTopicConfig {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.value.is_none() {
            return Err(SerializationError::NullValue(
                "value",
                _version,
                "CreateableTopicConfig",
            ));
        }
        if self.value.is_some() && self.value != Some(String::default()) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "value",
                _version,
                "CreateableTopicConfig",
            ));
        }
        Ok(())
    }
}
