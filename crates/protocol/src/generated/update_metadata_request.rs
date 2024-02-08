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

#[derive(Clone, Debug, Default)]
pub struct UpdateMetadataRequestTopicState {
    /// The topic name.
    pub topic_name: String,

    /// The partition that we would like to update.
    pub partition_states: Vec<UpdateMetadataPartitionState>,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct UpdateMetadataRequestBroker {
    pub id: i32,

    /// The broker hostname.
    pub v_0_host: String,

    /// The broker port.
    pub v_0_port: i32,

    /// The broker endpoints.
    pub endpoints: Vec<UpdateMetadataRequestEndpoint>,

    /// The rack which this broker belongs to.
    pub rack: Option<String>,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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
        self.controller_id.serialize(version, bytes)?;
        self.controller_epoch.serialize(version, bytes)?;
        if version >= 5 {
            self.broker_epoch.serialize(version, bytes)?;
        }
        if version >= 5 {
            self.topic_states.serialize(version, bytes)?;
        }
        if (0..=4).contains(&version) {
            self.partition_states_v_0.serialize(version, bytes)?;
        }
        self.brokers.serialize(version, bytes)?;
        Ok(())
    }
}

impl UpdateMetadataRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic_name.serialize(version, bytes)?;
        if version >= 5 {
            self.partition_states.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataRequestTopicState {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for UpdateMetadataRequestPartitionStateV0 {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if (0..=4).contains(&version) {
            self.topic_name.serialize(version, bytes)?;
        }
        if (0..=4).contains(&version) {
            self.partition_index.serialize(version, bytes)?;
        }
        if (0..=4).contains(&version) {
            self.controller_epoch.serialize(version, bytes)?;
        }
        if (0..=4).contains(&version) {
            self.leader.serialize(version, bytes)?;
        }
        if (0..=4).contains(&version) {
            self.leader_epoch.serialize(version, bytes)?;
        }
        if (0..=4).contains(&version) {
            self.isr.serialize(version, bytes)?;
        }
        if (0..=4).contains(&version) {
            self.zk_version.serialize(version, bytes)?;
        }
        if (0..=4).contains(&version) {
            self.replicas.serialize(version, bytes)?;
        }
        if version >= 4 {
            self.offline_replicas.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataRequestPartitionStateV0 {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for UpdateMetadataRequestBroker {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.id.serialize(version, bytes)?;
        if version >= 0 {
            self.v_0_host.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.v_0_port.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.endpoints.serialize(version, bytes)?;
        }
        if version >= 2 {
            self.rack.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataRequestBroker {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.rack.is_none() && !_version >= 2 {
            return Err(SerializationError::NullValue(
                "rack",
                _version,
                "UpdateMetadataRequestBroker",
            ));
        }
        Ok(())
    }
}

impl ToBytes for UpdateMetadataPartitionState {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 5 {
            self.partition_index.serialize(version, bytes)?;
        }
        if version >= 5 {
            self.controller_epoch.serialize(version, bytes)?;
        }
        if version >= 5 {
            self.leader.serialize(version, bytes)?;
        }
        if version >= 5 {
            self.leader_epoch.serialize(version, bytes)?;
        }
        if version >= 5 {
            self.isr.serialize(version, bytes)?;
        }
        if version >= 5 {
            self.zk_version.serialize(version, bytes)?;
        }
        if version >= 5 {
            self.replicas.serialize(version, bytes)?;
        }
        if version >= 5 {
            self.offline_replicas.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataPartitionState {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for UpdateMetadataRequestEndpoint {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 1 {
            self.port.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.host.serialize(version, bytes)?;
        }
        if version >= 3 {
            self.listener.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.security_protocol.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataRequestEndpoint {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
