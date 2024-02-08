use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AlterReplicaLogDirsRequest {
    /// The alterations to make for each directory.
    pub dirs: Vec<AlterReplicaLogDir>,
}

#[derive(Clone, Debug, Default)]
pub struct AlterReplicaLogDir {
    /// The absolute directory path.
    pub path: String,

    /// The topics to add to the directory.
    pub topics: Vec<AlterReplicaLogDirTopic>,
}

#[derive(Clone, Debug, Default)]
pub struct AlterReplicaLogDirTopic {
    /// The topic name.
    pub name: String,

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
        Ok(())
    }
}

impl ToBytes for AlterReplicaLogDir {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.path.serialize(version, bytes)?;
        self.topics.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDir {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for AlterReplicaLogDirTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl AlterReplicaLogDirTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
