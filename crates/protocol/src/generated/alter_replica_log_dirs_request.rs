use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlterReplicaLogDirsRequest {
    /// The alterations to make for each directory.
    pub dirs: BTreeMap<AlterReplicaLogDirKey, AlterReplicaLogDir>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Ord, PartialOrd)]
pub struct AlterReplicaLogDirKey {
    /// The absolute directory path.
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterReplicaLogDir {
    /// The topics to add to the directory.
    pub topics: BTreeMap<AlterReplicaLogDirTopicKey, AlterReplicaLogDirTopic>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Ord, PartialOrd)]
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

    fn get_api_key() -> i16 {
        34
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
        self.dirs.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.dirs != BTreeMap::<AlterReplicaLogDirKey, AlterReplicaLogDir>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "dirs",
                _version,
                "AlterReplicaLogDirsRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for AlterReplicaLogDirKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.path.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.path != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "path",
                _version,
                "AlterReplicaLogDirKey",
            ));
        }
        Ok(())
    }
}

impl ToBytes for AlterReplicaLogDir {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topics.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDir {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topics != BTreeMap::<AlterReplicaLogDirTopicKey, AlterReplicaLogDirTopic>::default()
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topics",
                _version,
                "AlterReplicaLogDir",
            ));
        }
        Ok(())
    }
}

impl ToBytes for AlterReplicaLogDirTopicKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirTopicKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "AlterReplicaLogDirTopicKey",
            ));
        }
        Ok(())
    }
}

impl ToBytes for AlterReplicaLogDirTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.partitions != Vec::<i32>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partitions",
                _version,
                "AlterReplicaLogDirTopic",
            ));
        }
        Ok(())
    }
}
