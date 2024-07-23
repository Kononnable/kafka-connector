use super::super::prelude::*;

/// Version 1 adds the broker epoch and reorganizes the partitions to be stored
/// per topic.
#[derive(Clone, Debug, PartialEq)]
pub struct StopReplicaRequest {
    /// The controller id.
    pub controller_id: i32,

    /// The controller epoch.
    pub controller_epoch: i32,

    /// The broker epoch.
    pub broker_epoch: i64,

    /// Whether these partitions should be deleted.
    pub delete_partitions: bool,

    /// The partitions to stop.
    pub partitions_v_0: Vec<StopReplicaRequestPartitionV0>,

    /// The topics to stop.
    pub topics: Vec<StopReplicaRequestTopic>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StopReplicaRequestPartitionV0 {
    /// The topic name.
    pub topic_name: String,

    /// The partition index.
    pub partition_index: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StopReplicaRequestTopic {
    /// The topic name.
    pub name: String,

    /// The partition indexes.
    pub partition_indexes: Vec<i32>,
}

impl ApiRequest for StopReplicaRequest {
    type Response = super::stop_replica_response::StopReplicaResponse;

    fn get_api_key() -> i16 {
        5
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
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
        self.controller_id.serialize(version, bytes)?;
        self.controller_epoch.serialize(version, bytes)?;
        if version >= 1 {
            self.broker_epoch.serialize(version, bytes)?;
        }
        self.delete_partitions.serialize(version, bytes)?;
        if version >= 0 {
            self.partitions_v_0.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.topics.serialize(version, bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let controller_id = i32::deserialize(version, bytes);
        let controller_epoch = i32::deserialize(version, bytes);
        let broker_epoch = if version >= 1 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let delete_partitions = bool::deserialize(version, bytes);
        let partitions_v_0 = if version >= 0 {
            Vec::<StopReplicaRequestPartitionV0>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 1 {
            Vec::<StopReplicaRequestTopic>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        StopReplicaRequest {
            controller_id,
            controller_epoch,
            broker_epoch,
            delete_partitions,
            partitions_v_0,
            topics,
        }
    }
}

impl StopReplicaRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.partitions_v_0 != Vec::<StopReplicaRequestPartitionV0>::default() && _version >= 0 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partitions_v_0",
                _version,
                "StopReplicaRequest",
            ));
        }
        if self.topics != Vec::<StopReplicaRequestTopic>::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topics",
                _version,
                "StopReplicaRequest",
            ));
        }
        Ok(())
    }
}

impl Default for StopReplicaRequest {
    fn default() -> Self {
        Self {
            controller_id: Default::default(),
            controller_epoch: Default::default(),
            broker_epoch: -1,
            delete_partitions: Default::default(),
            partitions_v_0: Default::default(),
            topics: Default::default(),
        }
    }
}

impl ToBytes for StopReplicaRequestPartitionV0 {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 0 {
            self.topic_name.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.partition_index.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl StopReplicaRequestPartitionV0 {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topic_name != String::default() && _version >= 0 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topic_name",
                _version,
                "StopReplicaRequestPartitionV0",
            ));
        }
        if self.partition_index != i32::default() && _version >= 0 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_index",
                _version,
                "StopReplicaRequestPartitionV0",
            ));
        }
        Ok(())
    }
}

impl FromBytes for StopReplicaRequestPartitionV0 {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let topic_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        StopReplicaRequestPartitionV0 {
            topic_name,
            partition_index,
        }
    }
}

impl ToBytes for StopReplicaRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 1 {
            self.name.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.partition_indexes.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl StopReplicaRequestTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "StopReplicaRequestTopic",
            ));
        }
        if self.partition_indexes != Vec::<i32>::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_indexes",
                _version,
                "StopReplicaRequestTopic",
            ));
        }
        Ok(())
    }
}

impl FromBytes for StopReplicaRequestTopic {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = if version >= 1 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_indexes = if version >= 1 {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        StopReplicaRequestTopic {
            name,
            partition_indexes,
        }
    }
}
