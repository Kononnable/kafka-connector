use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct UpdateMetadataRequest {
    /// The controller id.
    pub controller_id: i32,

    /// The controller epoch.
    pub controller_epoch: i32,

    /// The broker epoch.
    pub broker_epoch: i64,

    /// Each topic that we would like to update.
    pub topic_states: Vec<UpdateMetadataRequestTopicState>,

    /// Each partition that we would like to update.
    pub partition_states_v_0: Vec<UpdateMetadataRequestPartitionStateV0>,

    pub brokers: Vec<UpdateMetadataRequestBroker>,
}

#[derive(Debug, Clone)]
pub struct UpdateMetadataRequestTopicState {
    /// The topic name.
    pub topic_name: String,

    /// The partition that we would like to update.
    pub partition_states: Vec<UpdateMetadataPartitionState>,
}

#[derive(Debug, Clone)]
pub struct UpdateMetadataRequestPartitionStateV0 {
    /// The topic name.
    pub topic_name: String,

    /// The partition index.
    pub partition_index: i32,

    /// The controller epoch.
    pub controller_epoch: i32,

    /// The ID of the broker which is the current partition leader.
    pub leader: i32,

    /// The leader epoch of this partition.
    pub leader_epoch: i32,

    /// The brokers which are in the ISR for this partition.
    pub isr: Vec<i32>,

    /// The Zookeeper version.
    pub zk_version: i32,

    /// All the replicas of this partition.
    pub replicas: Vec<i32>,

    /// The replicas of this partition which are offline.
    pub offline_replicas: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct UpdateMetadataRequestBroker {
    pub id: i32,

    /// The broker hostname.
    pub v_0_host: String,

    /// The broker port.
    pub v_0_port: i32,

    /// The broker endpoints.
    pub endpoints: Vec<UpdateMetadataRequestEndpoint>,

    /// The rack which this broker belongs to.
    pub rack: String,
}

#[derive(Debug, Clone)]
pub struct UpdateMetadataPartitionState {
    /// The partition index.
    pub partition_index: i32,

    /// The controller epoch.
    pub controller_epoch: i32,

    /// The ID of the broker which is the current partition leader.
    pub leader: i32,

    /// The leader epoch of this partition.
    pub leader_epoch: i32,

    /// The brokers which are in the ISR for this partition.
    pub isr: Vec<i32>,

    /// The Zookeeper version.
    pub zk_version: i32,

    /// All the replicas of this partition.
    pub replicas: Vec<i32>,

    /// The replicas of this partition which are offline.
    pub offline_replicas: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct UpdateMetadataRequestEndpoint {
    /// The port of this endpoint
    pub port: i32,

    /// The hostname of this endpoint
    pub host: String,

    /// The listener name.
    pub listener: String,

    /// The security protocol type.
    pub security_protocol: i16,
}

impl ApiRequest for UpdateMetadataRequest {
    type Response = super::update_metadata_response::UpdateMetadataResponse;

    fn get_api_key() -> i16 {
        6
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        5
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
        if version >= 5 {
            self.broker_epoch.serialize(version, bytes);
        }
        if version >= 5 {
            self.topic_states.serialize(version, bytes);
        }
        if version >= 0 && version <= 4 {
            self.partition_states_v_0.serialize(version, bytes);
        }
        if version >= 0 {
            self.brokers.serialize(version, bytes);
        }
    }
}

impl Default for UpdateMetadataRequest {
    fn default() -> Self {
        Self {
            controller_id: Default::default(),
            controller_epoch: Default::default(),
            broker_epoch: -1,
            topic_states: Default::default(),
            partition_states_v_0: Default::default(),
            brokers: Default::default(),
        }
    }
}

impl ToBytes for UpdateMetadataRequestTopicState {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.topic_name.serialize(version, bytes);
        }
        if version >= 5 {
            self.partition_states.serialize(version, bytes);
        }
    }
}

impl Default for UpdateMetadataRequestTopicState {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_states: Default::default(),
        }
    }
}

impl ToBytes for UpdateMetadataRequestPartitionStateV0 {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 && version <= 4 {
            self.topic_name.serialize(version, bytes);
        }
        if version >= 0 && version <= 4 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 0 && version <= 4 {
            self.controller_epoch.serialize(version, bytes);
        }
        if version >= 0 && version <= 4 {
            self.leader.serialize(version, bytes);
        }
        if version >= 0 && version <= 4 {
            self.leader_epoch.serialize(version, bytes);
        }
        if version >= 0 && version <= 4 {
            self.isr.serialize(version, bytes);
        }
        if version >= 0 && version <= 4 {
            self.zk_version.serialize(version, bytes);
        }
        if version >= 0 && version <= 4 {
            self.replicas.serialize(version, bytes);
        }
        if version >= 4 {
            self.offline_replicas.serialize(version, bytes);
        }
    }
}

impl Default for UpdateMetadataRequestPartitionStateV0 {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_index: Default::default(),
            controller_epoch: Default::default(),
            leader: Default::default(),
            leader_epoch: Default::default(),
            isr: Default::default(),
            zk_version: Default::default(),
            replicas: Default::default(),
            offline_replicas: Default::default(),
        }
    }
}

impl ToBytes for UpdateMetadataRequestBroker {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.id.serialize(version, bytes);
        }
        if version >= 0 {
            self.v_0_host.serialize(version, bytes);
        }
        if version >= 0 {
            self.v_0_port.serialize(version, bytes);
        }
        if version >= 1 {
            self.endpoints.serialize(version, bytes);
        }
        if version >= 2 {
            self.rack.serialize(version, bytes);
        }
    }
}

impl Default for UpdateMetadataRequestBroker {
    fn default() -> Self {
        Self {
            id: Default::default(),
            v_0_host: Default::default(),
            v_0_port: Default::default(),
            endpoints: Default::default(),
            rack: Default::default(),
        }
    }
}

impl ToBytes for UpdateMetadataPartitionState {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 5 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 5 {
            self.controller_epoch.serialize(version, bytes);
        }
        if version >= 5 {
            self.leader.serialize(version, bytes);
        }
        if version >= 5 {
            self.leader_epoch.serialize(version, bytes);
        }
        if version >= 5 {
            self.isr.serialize(version, bytes);
        }
        if version >= 5 {
            self.zk_version.serialize(version, bytes);
        }
        if version >= 5 {
            self.replicas.serialize(version, bytes);
        }
        if version >= 5 {
            self.offline_replicas.serialize(version, bytes);
        }
    }
}

impl Default for UpdateMetadataPartitionState {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            controller_epoch: Default::default(),
            leader: Default::default(),
            leader_epoch: Default::default(),
            isr: Default::default(),
            zk_version: Default::default(),
            replicas: Default::default(),
            offline_replicas: Default::default(),
        }
    }
}

impl ToBytes for UpdateMetadataRequestEndpoint {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 1 {
            self.port.serialize(version, bytes);
        }
        if version >= 1 {
            self.host.serialize(version, bytes);
        }
        if version >= 3 {
            self.listener.serialize(version, bytes);
        }
        if version >= 1 {
            self.security_protocol.serialize(version, bytes);
        }
    }
}

impl Default for UpdateMetadataRequestEndpoint {
    fn default() -> Self {
        Self {
            port: Default::default(),
            host: Default::default(),
            listener: Default::default(),
            security_protocol: Default::default(),
        }
    }
}
