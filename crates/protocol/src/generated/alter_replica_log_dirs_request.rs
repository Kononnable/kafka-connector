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
        self.dirs.serialize(version, _bytes);
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
        for item in self.dirs.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for IndexMap<AlterReplicaLogDirKey, AlterReplicaLogDir> {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        _bytes.put_i32(self.len() as i32);
        for (key, value) in self {
            key.path.serialize(version, _bytes);
            value.topics.serialize(version, _bytes);
        }
    }
}

impl AlterReplicaLogDirKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl AlterReplicaLogDir {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for IndexMap<AlterReplicaLogDirKey, AlterReplicaLogDir> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        let mut ret = IndexMap::with_capacity(cap as usize);
        for _ in 0..cap {
            let path = String::deserialize(version, bytes);
            let topics =
                IndexMap::<AlterReplicaLogDirTopicKey, AlterReplicaLogDirTopic>::deserialize(
                    version, bytes,
                );
            let key = AlterReplicaLogDirKey { path };
            let value = AlterReplicaLogDir { topics };
            ret.insert(key, value);
        }

        ret
    }
}

impl ToBytes for IndexMap<AlterReplicaLogDirTopicKey, AlterReplicaLogDirTopic> {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        _bytes.put_i32(self.len() as i32);
        for (key, value) in self {
            key.name.serialize(version, _bytes);
            value.partitions.serialize(version, _bytes);
        }
    }
}

impl AlterReplicaLogDirTopicKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl AlterReplicaLogDirTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for IndexMap<AlterReplicaLogDirTopicKey, AlterReplicaLogDirTopic> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        let mut ret = IndexMap::with_capacity(cap as usize);
        for _ in 0..cap {
            let name = String::deserialize(version, bytes);
            let partitions = Vec::<i32>::deserialize(version, bytes);
            let key = AlterReplicaLogDirTopicKey { name };
            let value = AlterReplicaLogDirTopic { partitions };
            ret.insert(key, value);
        }

        ret
    }
}
