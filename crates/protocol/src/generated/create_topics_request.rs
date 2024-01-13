use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreateTopicsRequest {
    /// The topics to create.
    pub topics: Vec<CreatableTopic>,

    /// How long to wait in milliseconds before timing out the request.
    pub timeout_ms: i32,

    /// If true, check that the topics can be created as specified, but don't create anything.
    pub validate_only: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CreatableTopic {
    /// The topic name.
    pub name: String,

    /// The number of partitions to create in the topic, or -1 if we are specifying a manual partition assignment.
    pub num_partitions: i32,

    /// The number of replicas to create for each partition in the topic, or -1 if we are specifying a manual partition assignment.
    pub replication_factor: i16,

    /// The manual partition assignment, or the empty array if we are using automatic assignment.
    pub assignments: Vec<CreatableReplicaAssignment>,

    /// The custom topic configurations to set.
    pub configs: Vec<CreateableTopicConfig>,
}

#[derive(Clone, Debug, Default)]
pub struct CreatableReplicaAssignment {
    /// The partition index.
    pub partition_index: i32,

    /// The brokers to place the partition on.
    pub broker_ids: Vec<i32>,
}

#[derive(Clone, Debug, Default)]
pub struct CreateableTopicConfig {
    /// The configuration name.
    pub name: String,

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
        if version >= 0 {
            self.topics.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.timeout_ms.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.validate_only.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl CreateTopicsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatableTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 0 {
            self.name.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.num_partitions.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.replication_factor.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.assignments.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.configs.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl CreatableTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatableReplicaAssignment {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 0 {
            self.partition_index.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.broker_ids.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl CreatableReplicaAssignment {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreateableTopicConfig {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 0 {
            self.name.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.value.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl CreateableTopicConfig {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.value.is_none() && !_version >= 0 {
            return Err(SerializationError::NullValue(
                "value",
                _version,
                "CreateableTopicConfig",
            ));
        }
        Ok(())
    }
}
