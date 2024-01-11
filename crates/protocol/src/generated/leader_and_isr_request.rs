use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct LeaderAndIsrRequest {
    /// The current controller ID.
    pub controller_id: i32,

    /// The current controller epoch.
    pub controller_epoch: i32,

    /// The current broker epoch.
    pub broker_epoch: i64,

    /// Each topic.
    pub topic_states: Vec<LeaderAndIsrRequestTopicState>,

    /// The state of each partition
    pub partition_states_v_0: Vec<LeaderAndIsrRequestPartitionStateV0>,

    /// The current live leaders.
    pub live_leaders: Vec<LeaderAndIsrLiveLeader>,
}

#[derive(Debug, Clone)]
pub struct LeaderAndIsrRequestTopicState {
    /// The topic name.
    pub name: String,

    /// The state of each partition
    pub partition_states: Vec<LeaderAndIsrRequestPartitionState>,
}

#[derive(Debug, Clone)]
pub struct LeaderAndIsrRequestPartitionStateV0 {
    /// The topic name.
    pub topic_name: String,

    /// The partition index.
    pub partition_index: i32,

    /// The controller epoch.
    pub controller_epoch: i32,

    /// The broker ID of the leader.
    pub leader_key: i32,

    /// The leader epoch.
    pub leader_epoch: i32,

    /// The in-sync replica IDs.
    pub isr_replicas: Vec<i32>,

    /// The ZooKeeper version.
    pub zk_version: i32,

    /// The replica IDs.
    pub replicas: Vec<i32>,

    /// Whether the replica should have existed on the broker or not.
    pub is_new: bool,
}

#[derive(Debug, Clone)]
pub struct LeaderAndIsrLiveLeader {
    /// The leader's broker ID.
    pub broker_id: i32,

    /// The leader's hostname.
    pub host_name: String,

    /// The leader's port.
    pub port: i32,
}

#[derive(Debug, Clone)]
pub struct LeaderAndIsrRequestPartitionState {
    /// The partition index.
    pub partition_index: i32,

    /// The controller epoch.
    pub controller_epoch: i32,

    /// The broker ID of the leader.
    pub leader_key: i32,

    /// The leader epoch.
    pub leader_epoch: i32,

    /// The in-sync replica IDs.
    pub isr_replicas: Vec<i32>,

    /// The ZooKeeper version.
    pub zk_version: i32,

    /// The replica IDs.
    pub replicas: Vec<i32>,

    /// Whether the replica should have existed on the broker or not.
    pub is_new: bool,
}

impl ApiRequest for LeaderAndIsrRequest {
    type Response = super::leader_and_isr_response::LeaderAndIsrResponse;

    fn get_api_key() -> i16 {
        4
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
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
        if version >= 2 {
            self.broker_epoch.serialize(version, bytes);
        }
        if version >= 2 {
            self.topic_states.serialize(version, bytes);
        }
        if version >= 0 && version <= 1 {
            self.partition_states_v_0.serialize(version, bytes);
        }
        if version >= 0 {
            self.live_leaders.serialize(version, bytes);
        }
    }
}

impl Default for LeaderAndIsrRequest {
    fn default() -> Self {
        Self {
            controller_id: Default::default(),
            controller_epoch: Default::default(),
            broker_epoch: -1,
            topic_states: Default::default(),
            partition_states_v_0: Default::default(),
            live_leaders: Default::default(),
        }
    }
}

impl ToBytes for LeaderAndIsrRequestTopicState {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 2 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partition_states.serialize(version, bytes);
        }
    }
}

impl Default for LeaderAndIsrRequestTopicState {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_states: Default::default(),
        }
    }
}

impl ToBytes for LeaderAndIsrRequestPartitionStateV0 {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 && version <= 1 {
            self.topic_name.serialize(version, bytes);
        }
        if version >= 0 && version <= 1 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 0 && version <= 1 {
            self.controller_epoch.serialize(version, bytes);
        }
        if version >= 0 && version <= 1 {
            self.leader_key.serialize(version, bytes);
        }
        if version >= 0 && version <= 1 {
            self.leader_epoch.serialize(version, bytes);
        }
        if version >= 0 && version <= 1 {
            self.isr_replicas.serialize(version, bytes);
        }
        if version >= 0 && version <= 1 {
            self.zk_version.serialize(version, bytes);
        }
        if version >= 0 && version <= 1 {
            self.replicas.serialize(version, bytes);
        }
        if version >= 1 {
            self.is_new.serialize(version, bytes);
        }
    }
}

impl Default for LeaderAndIsrRequestPartitionStateV0 {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_index: Default::default(),
            controller_epoch: Default::default(),
            leader_key: Default::default(),
            leader_epoch: Default::default(),
            isr_replicas: Default::default(),
            zk_version: Default::default(),
            replicas: Default::default(),
            is_new: false,
        }
    }
}

impl ToBytes for LeaderAndIsrLiveLeader {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.broker_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.host_name.serialize(version, bytes);
        }
        if version >= 0 {
            self.port.serialize(version, bytes);
        }
    }
}

impl Default for LeaderAndIsrLiveLeader {
    fn default() -> Self {
        Self {
            broker_id: Default::default(),
            host_name: Default::default(),
            port: Default::default(),
        }
    }
}

impl ToBytes for LeaderAndIsrRequestPartitionState {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 0 {
            self.controller_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.leader_key.serialize(version, bytes);
        }
        if version >= 0 {
            self.leader_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.isr_replicas.serialize(version, bytes);
        }
        if version >= 0 {
            self.zk_version.serialize(version, bytes);
        }
        if version >= 0 {
            self.replicas.serialize(version, bytes);
        }
        if version >= 1 {
            self.is_new.serialize(version, bytes);
        }
    }
}

impl Default for LeaderAndIsrRequestPartitionState {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            controller_epoch: Default::default(),
            leader_key: Default::default(),
            leader_epoch: Default::default(),
            isr_replicas: Default::default(),
            zk_version: Default::default(),
            replicas: Default::default(),
            is_new: false,
        }
    }
}
