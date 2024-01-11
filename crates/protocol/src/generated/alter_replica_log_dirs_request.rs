use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AlterReplicaLogDirsRequest {
    pub dirs: Vec<AlterReplicaLogDir>,
}

#[derive(Debug, Default, Clone)]
pub struct AlterReplicaLogDir {
    pub path: String,
    pub topics: Vec<AlterReplicaLogDirTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct AlterReplicaLogDirTopic {
    pub name: String,
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.dirs.serialize(version, bytes);
        }
    }
}

impl ToBytes for AlterReplicaLogDir {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.path.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
    }
}

impl ToBytes for AlterReplicaLogDirTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}
