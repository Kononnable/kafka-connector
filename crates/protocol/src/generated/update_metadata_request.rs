use super::super::prelude::*;

/// Version 1 allows specifying multiple endpoints for each broker.
///
/// Version 2 adds the rack.
///
/// Version 3 adds the listener name.
///
/// Version 4 adds the offline replica list.
///
/// Version 5 adds the broker epoch field and normalizes partitions by topic.
/// Version 0 of the protocol only allowed specifying a single host and
/// port per broker, rather than an array of endpoints.
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct UpdateMetadataRequestTopicState {
    /// The topic name.
    pub topic_name: String,

    /// The partition that we would like to update.
    pub partition_states: Vec<UpdateMetadataPartitionState>,
}

#[derive(Clone, Debug, PartialEq, Default)]
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

#[derive(Clone, Debug, PartialEq, Default)]
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

#[derive(Clone, Debug, PartialEq, Default)]
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

#[derive(Clone, Debug, PartialEq, Default)]
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

    fn get_api_key() -> ApiKey {
        ApiKey(6)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(5)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.controller_id.serialize(version, _bytes)?;
        self.controller_epoch.serialize(version, _bytes)?;
        if version >= ApiVersion(5) {
            self.broker_epoch.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.topic_states.serialize(version, _bytes)?;
        }
        if (0..=4).contains(&version.0) {
            self.partition_states_v_0.serialize(version, _bytes)?;
        }
        self.brokers.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let controller_id = i32::deserialize(version, bytes);
        let controller_epoch = i32::deserialize(version, bytes);
        let broker_epoch = if version >= ApiVersion(5) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topic_states = if version >= ApiVersion(5) {
            Vec::<UpdateMetadataRequestTopicState>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_states_v_0 = if (0..=4).contains(&version.0) {
            Vec::<UpdateMetadataRequestPartitionStateV0>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let brokers = Vec::<UpdateMetadataRequestBroker>::deserialize(version, bytes);
        UpdateMetadataRequest {
            controller_id,
            controller_epoch,
            broker_epoch,
            topic_states,
            partition_states_v_0,
            brokers,
        }
    }
}

impl UpdateMetadataRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.topic_states != Vec::<UpdateMetadataRequestTopicState>::default()
            && _version >= ApiVersion(5)
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topic_states",
                *_version,
                "UpdateMetadataRequest",
            ));
        }
        if self.partition_states_v_0 != Vec::<UpdateMetadataRequestPartitionStateV0>::default()
            && !(0..=4).contains(&_version.0)
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_states_v_0",
                *_version,
                "UpdateMetadataRequest",
            ));
        }
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
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic_name.serialize(version, _bytes)?;
        if version >= ApiVersion(5) {
            self.partition_states.serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataRequestTopicState {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.partition_states != Vec::<UpdateMetadataPartitionState>::default()
            && _version >= ApiVersion(5)
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_states",
                *_version,
                "UpdateMetadataRequestTopicState",
            ));
        }
        Ok(())
    }
}

impl FromBytes for UpdateMetadataRequestTopicState {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic_name = String::deserialize(version, bytes);
        let partition_states = if version >= ApiVersion(5) {
            Vec::<UpdateMetadataPartitionState>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        UpdateMetadataRequestTopicState {
            topic_name,
            partition_states,
        }
    }
}

impl ToBytes for UpdateMetadataRequestPartitionStateV0 {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if (0..=4).contains(&version.0) {
            self.topic_name.serialize(version, _bytes)?;
        }
        if (0..=4).contains(&version.0) {
            self.partition_index.serialize(version, _bytes)?;
        }
        if (0..=4).contains(&version.0) {
            self.controller_epoch.serialize(version, _bytes)?;
        }
        if (0..=4).contains(&version.0) {
            self.leader.serialize(version, _bytes)?;
        }
        if (0..=4).contains(&version.0) {
            self.leader_epoch.serialize(version, _bytes)?;
        }
        if (0..=4).contains(&version.0) {
            self.isr.serialize(version, _bytes)?;
        }
        if (0..=4).contains(&version.0) {
            self.zk_version.serialize(version, _bytes)?;
        }
        if (0..=4).contains(&version.0) {
            self.replicas.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(4) {
            self.offline_replicas.serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataRequestPartitionStateV0 {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.topic_name != String::default() && !(0..=4).contains(&_version.0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topic_name",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        if self.partition_index != i32::default() && !(0..=4).contains(&_version.0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_index",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        if self.controller_epoch != i32::default() && !(0..=4).contains(&_version.0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "controller_epoch",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        if self.leader != i32::default() && !(0..=4).contains(&_version.0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "leader",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        if self.leader_epoch != i32::default() && !(0..=4).contains(&_version.0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "leader_epoch",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        if self.isr != Vec::<i32>::default() && !(0..=4).contains(&_version.0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "isr",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        if self.zk_version != i32::default() && !(0..=4).contains(&_version.0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "zk_version",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        if self.replicas != Vec::<i32>::default() && !(0..=4).contains(&_version.0) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "replicas",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        if self.offline_replicas != Vec::<i32>::default() && _version >= ApiVersion(4) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "offline_replicas",
                *_version,
                "UpdateMetadataRequestPartitionStateV0",
            ));
        }
        Ok(())
    }
}

impl FromBytes for UpdateMetadataRequestPartitionStateV0 {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic_name = if (0..=4).contains(&version.0) {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_index = if (0..=4).contains(&version.0) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let controller_epoch = if (0..=4).contains(&version.0) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader = if (0..=4).contains(&version.0) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = if (0..=4).contains(&version.0) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let isr = if (0..=4).contains(&version.0) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let zk_version = if (0..=4).contains(&version.0) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let replicas = if (0..=4).contains(&version.0) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let offline_replicas = if version >= ApiVersion(4) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        UpdateMetadataRequestPartitionStateV0 {
            topic_name,
            partition_index,
            controller_epoch,
            leader,
            leader_epoch,
            isr,
            zk_version,
            replicas,
            offline_replicas,
        }
    }
}

impl ToBytes for UpdateMetadataRequestBroker {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.id.serialize(version, _bytes)?;
        if version >= ApiVersion(0) {
            self.v_0_host.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(0) {
            self.v_0_port.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(1) {
            self.endpoints.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(2) {
            self.rack.serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataRequestBroker {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.endpoints != Vec::<UpdateMetadataRequestEndpoint>::default()
            && _version >= ApiVersion(1)
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "endpoints",
                *_version,
                "UpdateMetadataRequestBroker",
            ));
        }
        Ok(())
    }
}

impl FromBytes for UpdateMetadataRequestBroker {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let id = i32::deserialize(version, bytes);
        let v_0_host = if version >= ApiVersion(0) {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let v_0_port = if version >= ApiVersion(0) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let endpoints = if version >= ApiVersion(1) {
            Vec::<UpdateMetadataRequestEndpoint>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let rack = if version >= ApiVersion(2) {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        UpdateMetadataRequestBroker {
            id,
            v_0_host,
            v_0_port,
            endpoints,
            rack,
        }
    }
}

impl ToBytes for UpdateMetadataPartitionState {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= ApiVersion(5) {
            self.partition_index.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.controller_epoch.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.leader.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.leader_epoch.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.isr.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.zk_version.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.replicas.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.offline_replicas.serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataPartitionState {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.partition_index != i32::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_index",
                *_version,
                "UpdateMetadataPartitionState",
            ));
        }
        if self.controller_epoch != i32::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "controller_epoch",
                *_version,
                "UpdateMetadataPartitionState",
            ));
        }
        if self.leader != i32::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "leader",
                *_version,
                "UpdateMetadataPartitionState",
            ));
        }
        if self.leader_epoch != i32::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "leader_epoch",
                *_version,
                "UpdateMetadataPartitionState",
            ));
        }
        if self.isr != Vec::<i32>::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "isr",
                *_version,
                "UpdateMetadataPartitionState",
            ));
        }
        if self.zk_version != i32::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "zk_version",
                *_version,
                "UpdateMetadataPartitionState",
            ));
        }
        if self.replicas != Vec::<i32>::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "replicas",
                *_version,
                "UpdateMetadataPartitionState",
            ));
        }
        if self.offline_replicas != Vec::<i32>::default() && _version >= ApiVersion(5) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "offline_replicas",
                *_version,
                "UpdateMetadataPartitionState",
            ));
        }
        Ok(())
    }
}

impl FromBytes for UpdateMetadataPartitionState {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = if version >= ApiVersion(5) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let controller_epoch = if version >= ApiVersion(5) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader = if version >= ApiVersion(5) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = if version >= ApiVersion(5) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let isr = if version >= ApiVersion(5) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let zk_version = if version >= ApiVersion(5) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let replicas = if version >= ApiVersion(5) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let offline_replicas = if version >= ApiVersion(5) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        UpdateMetadataPartitionState {
            partition_index,
            controller_epoch,
            leader,
            leader_epoch,
            isr,
            zk_version,
            replicas,
            offline_replicas,
        }
    }
}

impl ToBytes for UpdateMetadataRequestEndpoint {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= ApiVersion(1) {
            self.port.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(1) {
            self.host.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(3) {
            self.listener.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(1) {
            self.security_protocol.serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl UpdateMetadataRequestEndpoint {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.port != i32::default() && _version >= ApiVersion(1) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "port",
                *_version,
                "UpdateMetadataRequestEndpoint",
            ));
        }
        if self.host != String::default() && _version >= ApiVersion(1) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "host",
                *_version,
                "UpdateMetadataRequestEndpoint",
            ));
        }
        if self.listener != String::default() && _version >= ApiVersion(3) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "listener",
                *_version,
                "UpdateMetadataRequestEndpoint",
            ));
        }
        if self.security_protocol != i16::default() && _version >= ApiVersion(1) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "security_protocol",
                *_version,
                "UpdateMetadataRequestEndpoint",
            ));
        }
        Ok(())
    }
}

impl FromBytes for UpdateMetadataRequestEndpoint {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let port = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let host = if version >= ApiVersion(1) {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let listener = if version >= ApiVersion(3) {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let security_protocol = if version >= ApiVersion(1) {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        UpdateMetadataRequestEndpoint {
            port,
            host,
            listener,
            security_protocol,
        }
    }
}
