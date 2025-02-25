use super::super::prelude::*;

/// Version 1 adds fields for the rack of each broker, the controller id, and
/// whether or not the topic is internal.
///
/// Version 2 adds the cluster ID field.
///
/// Version 3 adds the throttle time.
///
/// Version 4 is the same as version 3.
///
/// Version 5 adds a per-partition offline_replicas field. This field specifies
/// the list of replicas that are offline.
///
/// Starting in version 6, on quota violation, brokers send out responses before throttling.
///
/// Version 7 adds the leader epoch to the partition metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct MetadataResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each broker in the response.
    pub brokers: IndexMap<MetadataResponseBrokerKey, MetadataResponseBroker>,

    /// The cluster ID that responding broker belongs to.
    pub cluster_id: Option<String>,

    /// The ID of the controller broker.
    pub controller_id: i32,

    /// Each topic in the response.
    pub topics: IndexMap<MetadataResponseTopicKey, MetadataResponseTopic>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct MetadataResponseBrokerKey {
    /// The broker ID.
    pub node_id: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MetadataResponseBroker {
    /// The broker hostname.
    pub host: String,

    /// The broker port.
    pub port: i32,

    /// The rack of the broker, or null if it has not been assigned to a rack.
    pub rack: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct MetadataResponseTopicKey {
    /// The topic name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MetadataResponseTopic {
    /// The topic error, or 0 if there was no error.
    pub error_code: i16,

    /// True if the topic is internal.
    pub is_internal: bool,

    /// Each partition in the topic.
    pub partitions: Vec<MetadataResponsePartition>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataResponsePartition {
    /// The partition error, or 0 if there was no error.
    pub error_code: i16,

    /// The partition index.
    pub partition_index: i32,

    /// The ID of the leader broker.
    pub leader_id: i32,

    /// The leader epoch of this partition.
    pub leader_epoch: i32,

    /// The set of all nodes that host this partition.
    pub replica_nodes: Vec<i32>,

    /// The set of nodes that are in sync with the leader for this partition.
    pub isr_nodes: Vec<i32>,

    /// The set of offline replicas of this partition.
    pub offline_replicas: Vec<i32>,
}

impl ApiResponse for MetadataResponse {
    type Request = super::metadata_request::MetadataRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(3)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(7)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        if version >= ApiVersion(3) {
            self.throttle_time_ms.serialize(version, _bytes);
        }
        self.brokers.serialize(version, _bytes);
        if version >= ApiVersion(2) {
            self.cluster_id.serialize(version, _bytes);
        }
        if version >= ApiVersion(1) {
            self.controller_id.serialize(version, _bytes);
        }
        self.topics.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(3) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let brokers = IndexMap::<MetadataResponseBrokerKey, MetadataResponseBroker>::deserialize(
            version, bytes,
        );
        let cluster_id = if version >= ApiVersion(2) {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let controller_id = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = IndexMap::<MetadataResponseTopicKey, MetadataResponseTopic>::deserialize(
            version, bytes,
        );
        MetadataResponse {
            throttle_time_ms,
            brokers,
            cluster_id,
            controller_id,
            topics,
        }
    }
}

impl MetadataResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.brokers.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        for item in self.topics.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        if self.throttle_time_ms != i32::default() && _version.0 < 3 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "throttle_time_ms",
                *_version,
                "MetadataResponse",
            ));
        }
        Ok(())
    }
}

impl Default for MetadataResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            brokers: Default::default(),
            cluster_id: Default::default(),
            controller_id: -1,
            topics: Default::default(),
        }
    }
}

impl ToBytes for MetadataResponseBrokerKey {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.node_id.serialize(version, _bytes);
    }
}

impl MetadataResponseBrokerKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataResponseBrokerKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let node_id = i32::deserialize(version, bytes);
        MetadataResponseBrokerKey { node_id }
    }
}

impl ToBytes for MetadataResponseBroker {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.host.serialize(version, _bytes);
        self.port.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.rack.serialize(version, _bytes);
        }
    }
}

impl MetadataResponseBroker {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataResponseBroker {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let host = String::deserialize(version, bytes);
        let port = i32::deserialize(version, bytes);
        let rack = if version >= ApiVersion(1) {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        MetadataResponseBroker { host, port, rack }
    }
}

impl ToBytes for MetadataResponseTopicKey {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
    }
}

impl MetadataResponseTopicKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataResponseTopicKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        MetadataResponseTopicKey { name }
    }
}

impl ToBytes for MetadataResponseTopic {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.error_code.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.is_internal.serialize(version, _bytes);
        }
        self.partitions.serialize(version, _bytes);
    }
}

impl MetadataResponseTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.partitions.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for MetadataResponseTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let is_internal = if version >= ApiVersion(1) {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = Vec::<MetadataResponsePartition>::deserialize(version, bytes);
        MetadataResponseTopic {
            error_code,
            is_internal,
            partitions,
        }
    }
}

impl ToBytes for MetadataResponsePartition {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.error_code.serialize(version, _bytes);
        self.partition_index.serialize(version, _bytes);
        self.leader_id.serialize(version, _bytes);
        if version >= ApiVersion(7) {
            self.leader_epoch.serialize(version, _bytes);
        }
        self.replica_nodes.serialize(version, _bytes);
        self.isr_nodes.serialize(version, _bytes);
        if version >= ApiVersion(5) {
            self.offline_replicas.serialize(version, _bytes);
        }
    }
}

impl MetadataResponsePartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataResponsePartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        let leader_id = i32::deserialize(version, bytes);
        let leader_epoch = if version >= ApiVersion(7) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let replica_nodes = Vec::<i32>::deserialize(version, bytes);
        let isr_nodes = Vec::<i32>::deserialize(version, bytes);
        let offline_replicas = if version >= ApiVersion(5) {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        MetadataResponsePartition {
            error_code,
            partition_index,
            leader_id,
            leader_epoch,
            replica_nodes,
            isr_nodes,
            offline_replicas,
        }
    }
}

impl Default for MetadataResponsePartition {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            partition_index: Default::default(),
            leader_id: Default::default(),
            leader_epoch: -1,
            replica_nodes: Default::default(),
            isr_nodes: Default::default(),
            offline_replicas: Default::default(),
        }
    }
}
