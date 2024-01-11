use super::super::prelude::*;

#[derive(Clone, Debug)]
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

#[derive(Debug, Clone)]
pub struct StopReplicaRequestPartitionV0 {
    /// The topic name.
    pub topic_name: String,

    /// The partition index.
    pub partition_index: i32,
}

#[derive(Debug, Clone)]
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.controller_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.controller_epoch.serialize(version, bytes);
        }
        if version >= 1 {
            self.broker_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.delete_partitions.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions_v_0.serialize(version, bytes);
        }
        if version >= 1 {
            self.topics.serialize(version, bytes);
        }
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.topic_name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
    }
}

impl Default for StopReplicaRequestPartitionV0 {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_index: Default::default(),
        }
    }
}

impl ToBytes for StopReplicaRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 1 {
            self.name.serialize(version, bytes);
        }
        if version >= 1 {
            self.partition_indexes.serialize(version, bytes);
        }
    }
}

impl Default for StopReplicaRequestTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_indexes: Default::default(),
        }
    }
}
