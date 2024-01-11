use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct LeaderAndIsrRequest {
    pub controller_id: i32,
    pub controller_epoch: i32,
    pub broker_epoch: i64,
    pub topic_states: Vec<LeaderAndIsrRequestTopicState>,
    pub partition_states_v_0: Vec<LeaderAndIsrRequestPartitionStateV0>,
    pub live_leaders: Vec<LeaderAndIsrLiveLeader>,
}

#[derive(Debug, Default, Clone)]
pub struct LeaderAndIsrRequestTopicState {
    pub name: String,
    pub partition_states: Vec<LeaderAndIsrRequestPartitionState>,
}

#[derive(Debug, Default, Clone)]
pub struct LeaderAndIsrRequestPartitionStateV0 {
    pub topic_name: String,
    pub partition_index: i32,
    pub controller_epoch: i32,
    pub leader_key: i32,
    pub leader_epoch: i32,
    pub isr_replicas: Vec<i32>,
    pub zk_version: i32,
    pub replicas: Vec<i32>,
    pub is_new: bool,
}

#[derive(Debug, Default, Clone)]
pub struct LeaderAndIsrLiveLeader {
    pub broker_id: i32,
    pub host_name: String,
    pub port: i32,
}

#[derive(Debug, Default, Clone)]
pub struct LeaderAndIsrRequestPartitionState {
    pub partition_index: i32,
    pub controller_epoch: i32,
    pub leader_key: i32,
    pub leader_epoch: i32,
    pub isr_replicas: Vec<i32>,
    pub zk_version: i32,
    pub replicas: Vec<i32>,
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
