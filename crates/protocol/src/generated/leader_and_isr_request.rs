use super::super::prelude::*;

/// Version 1 adds IsNew.
///
/// Version 2 adds broker epoch and reorganizes the partitions by topic.
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LeaderAndIsrRequestTopicState {
    /// The topic name.
    pub name: String,

    /// The state of each partition
    pub partition_states: Vec<LeaderAndIsrRequestPartitionState>,
}

#[derive(Clone, Debug, PartialEq, Default)]
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LeaderAndIsrLiveLeader {
    /// The leader's broker ID.
    pub broker_id: i32,

    /// The leader's hostname.
    pub host_name: String,

    /// The leader's port.
    pub port: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
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
        if version >= 2 {
            self.broker_epoch.serialize(version, bytes)?;
        }
        if version >= 2 {
            self.topic_states.serialize(version, bytes)?;
        }
        if (0..=1).contains(&version) {
            self.partition_states_v_0.serialize(version, bytes)?;
        }
        self.live_leaders.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let controller_id = i32::deserialize(version, bytes);
        let controller_epoch = i32::deserialize(version, bytes);
        let broker_epoch = if version >= 2 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topic_states = if version >= 2 {
            Vec::<LeaderAndIsrRequestTopicState>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_states_v_0 = if (0..=1).contains(&version) {
            Vec::<LeaderAndIsrRequestPartitionStateV0>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let live_leaders = Vec::<LeaderAndIsrLiveLeader>::deserialize(version, bytes);
        LeaderAndIsrRequest {
            controller_id,
            controller_epoch,
            broker_epoch,
            topic_states,
            partition_states_v_0,
            live_leaders,
        }
    }
}

impl LeaderAndIsrRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topic_states != Vec::<LeaderAndIsrRequestTopicState>::default() && _version >= 2 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topic_states",
                _version,
                "LeaderAndIsrRequest",
            ));
        }
        if self.partition_states_v_0 != Vec::<LeaderAndIsrRequestPartitionStateV0>::default()
            && !(0..=1).contains(&_version)
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_states_v_0",
                _version,
                "LeaderAndIsrRequest",
            ));
        }
        Ok(())
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 2 {
            self.name.serialize(version, bytes)?;
        }
        self.partition_states.serialize(version, bytes)?;
        Ok(())
    }
}

impl LeaderAndIsrRequestTopicState {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() && _version >= 2 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "LeaderAndIsrRequestTopicState",
            ));
        }
        Ok(())
    }
}

impl FromBytes for LeaderAndIsrRequestTopicState {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = if version >= 2 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_states =
            Vec::<LeaderAndIsrRequestPartitionState>::deserialize(version, bytes);
        LeaderAndIsrRequestTopicState {
            name,
            partition_states,
        }
    }
}

impl ToBytes for LeaderAndIsrRequestPartitionStateV0 {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if (0..=1).contains(&version) {
            self.topic_name.serialize(version, bytes)?;
        }
        if (0..=1).contains(&version) {
            self.partition_index.serialize(version, bytes)?;
        }
        if (0..=1).contains(&version) {
            self.controller_epoch.serialize(version, bytes)?;
        }
        if (0..=1).contains(&version) {
            self.leader_key.serialize(version, bytes)?;
        }
        if (0..=1).contains(&version) {
            self.leader_epoch.serialize(version, bytes)?;
        }
        if (0..=1).contains(&version) {
            self.isr_replicas.serialize(version, bytes)?;
        }
        if (0..=1).contains(&version) {
            self.zk_version.serialize(version, bytes)?;
        }
        if (0..=1).contains(&version) {
            self.replicas.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.is_new.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl LeaderAndIsrRequestPartitionStateV0 {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topic_name != String::default() && !(0..=1).contains(&_version) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topic_name",
                _version,
                "LeaderAndIsrRequestPartitionStateV0",
            ));
        }
        if self.partition_index != i32::default() && !(0..=1).contains(&_version) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_index",
                _version,
                "LeaderAndIsrRequestPartitionStateV0",
            ));
        }
        if self.controller_epoch != i32::default() && !(0..=1).contains(&_version) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "controller_epoch",
                _version,
                "LeaderAndIsrRequestPartitionStateV0",
            ));
        }
        if self.leader_key != i32::default() && !(0..=1).contains(&_version) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "leader_key",
                _version,
                "LeaderAndIsrRequestPartitionStateV0",
            ));
        }
        if self.leader_epoch != i32::default() && !(0..=1).contains(&_version) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "leader_epoch",
                _version,
                "LeaderAndIsrRequestPartitionStateV0",
            ));
        }
        if self.isr_replicas != Vec::<i32>::default() && !(0..=1).contains(&_version) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "isr_replicas",
                _version,
                "LeaderAndIsrRequestPartitionStateV0",
            ));
        }
        if self.zk_version != i32::default() && !(0..=1).contains(&_version) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "zk_version",
                _version,
                "LeaderAndIsrRequestPartitionStateV0",
            ));
        }
        if self.replicas != Vec::<i32>::default() && !(0..=1).contains(&_version) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "replicas",
                _version,
                "LeaderAndIsrRequestPartitionStateV0",
            ));
        }
        Ok(())
    }
}

impl FromBytes for LeaderAndIsrRequestPartitionStateV0 {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let topic_name = if (0..=1).contains(&version) {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_index = if (0..=1).contains(&version) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let controller_epoch = if (0..=1).contains(&version) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_key = if (0..=1).contains(&version) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = if (0..=1).contains(&version) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let isr_replicas = if (0..=1).contains(&version) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let zk_version = if (0..=1).contains(&version) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let replicas = if (0..=1).contains(&version) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let is_new = if version >= 1 {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        LeaderAndIsrRequestPartitionStateV0 {
            topic_name,
            partition_index,
            controller_epoch,
            leader_key,
            leader_epoch,
            isr_replicas,
            zk_version,
            replicas,
            is_new,
        }
    }
}

impl ToBytes for LeaderAndIsrLiveLeader {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.broker_id.serialize(version, bytes)?;
        self.host_name.serialize(version, bytes)?;
        self.port.serialize(version, bytes)?;
        Ok(())
    }
}

impl LeaderAndIsrLiveLeader {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for LeaderAndIsrLiveLeader {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let broker_id = i32::deserialize(version, bytes);
        let host_name = String::deserialize(version, bytes);
        let port = i32::deserialize(version, bytes);
        LeaderAndIsrLiveLeader {
            broker_id,
            host_name,
            port,
        }
    }
}

impl ToBytes for LeaderAndIsrRequestPartitionState {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.controller_epoch.serialize(version, bytes)?;
        self.leader_key.serialize(version, bytes)?;
        self.leader_epoch.serialize(version, bytes)?;
        self.isr_replicas.serialize(version, bytes)?;
        self.zk_version.serialize(version, bytes)?;
        self.replicas.serialize(version, bytes)?;
        if version >= 1 {
            self.is_new.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl LeaderAndIsrRequestPartitionState {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for LeaderAndIsrRequestPartitionState {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let controller_epoch = i32::deserialize(version, bytes);
        let leader_key = i32::deserialize(version, bytes);
        let leader_epoch = i32::deserialize(version, bytes);
        let isr_replicas = Vec::<i32>::deserialize(version, bytes);
        let zk_version = i32::deserialize(version, bytes);
        let replicas = Vec::<i32>::deserialize(version, bytes);
        let is_new = if version >= 1 {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        LeaderAndIsrRequestPartitionState {
            partition_index,
            controller_epoch,
            leader_key,
            leader_epoch,
            isr_replicas,
            zk_version,
            replicas,
            is_new,
        }
    }
}
