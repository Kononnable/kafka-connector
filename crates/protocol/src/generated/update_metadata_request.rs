use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct UpdateMetadataRequest {
    pub controller_id: i32,
    pub controller_epoch: i32,
    pub broker_epoch: i64,
    pub topic_states: Vec<UpdateMetadataRequestTopicState>,
    pub partition_states_v_0: Vec<UpdateMetadataRequestPartitionStateV0>,
    pub brokers: Vec<UpdateMetadataRequestBroker>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateMetadataRequestTopicState {
    pub topic_name: String,
    pub partition_states: Vec<UpdateMetadataPartitionState>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateMetadataRequestPartitionStateV0 {
    pub topic_name: String,
    pub partition_index: i32,
    pub controller_epoch: i32,
    pub leader: i32,
    pub leader_epoch: i32,
    pub isr: Vec<i32>,
    pub zk_version: i32,
    pub replicas: Vec<i32>,
    pub offline_replicas: Vec<i32>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateMetadataRequestBroker {
    pub id: i32,
    pub v_0_host: String,
    pub v_0_port: i32,
    pub endpoints: Vec<UpdateMetadataRequestEndpoint>,
    pub rack: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateMetadataPartitionState {
    pub partition_index: i32,
    pub controller_epoch: i32,
    pub leader: i32,
    pub leader_epoch: i32,
    pub isr: Vec<i32>,
    pub zk_version: i32,
    pub replicas: Vec<i32>,
    pub offline_replicas: Vec<i32>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateMetadataRequestEndpoint {
    pub port: i32,
    pub host: String,
    pub listener: String,
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
