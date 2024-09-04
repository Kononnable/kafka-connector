use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlterReplicaLogDirsRequest {
    /// The alterations to make for each directory.
    pub dirs: IndexMap<AlterReplicaLogDirKey, AlterReplicaLogDir>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AlterReplicaLogDirKey {
    /// The absolute directory path.
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterReplicaLogDir {
    /// The topics to add to the directory.
    pub topics: IndexMap<AlterReplicaLogDirTopicKey, AlterReplicaLogDirTopic>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AlterReplicaLogDirTopicKey {
    /// The topic name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterReplicaLogDirTopic {
    /// The partition indexes.
    pub partitions: Vec<i32>,
}

impl ApiRequest for AlterReplicaLogDirsRequest {
    type Response = super::alter_replica_log_dirs_response::AlterReplicaLogDirsResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(34)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(1)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.dirs.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let dirs =
            IndexMap::<AlterReplicaLogDirKey, AlterReplicaLogDir>::deserialize(version, bytes);
        AlterReplicaLogDirsRequest { dirs }
    }
}

impl AlterReplicaLogDirsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for AlterReplicaLogDirKey {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.path.serialize(version, _bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterReplicaLogDirKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let path = String::deserialize(version, bytes);
        AlterReplicaLogDirKey { path }
    }
}

impl ToBytes for AlterReplicaLogDir {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDir {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterReplicaLogDir {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topics = IndexMap::<AlterReplicaLogDirTopicKey, AlterReplicaLogDirTopic>::deserialize(
            version, bytes,
        );
        AlterReplicaLogDir { topics }
    }
}

impl ToBytes for AlterReplicaLogDirTopicKey {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirTopicKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterReplicaLogDirTopicKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        AlterReplicaLogDirTopicKey { name }
    }
}

impl ToBytes for AlterReplicaLogDirTopic {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partitions.serialize(version, _bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AlterReplicaLogDirTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partitions = Vec::<i32>::deserialize(version, bytes);
        AlterReplicaLogDirTopic { partitions }
    }
}
