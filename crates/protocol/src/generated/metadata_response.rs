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

    fn get_api_key() -> i16 {
        3
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        7
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &ResponseHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        if version >= 3 {
            self.throttle_time_ms.serialize(version, bytes)?;
        }
        self.brokers.serialize(version, bytes)?;
        if version >= 2 {
            self.cluster_id.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.controller_id.serialize(version, bytes)?;
        }
        self.topics.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 3 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let brokers = IndexMap::<MetadataResponseBrokerKey, MetadataResponseBroker>::deserialize(
            version, bytes,
        );
        let cluster_id = if version >= 2 {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let controller_id = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = IndexMap::<MetadataResponseTopicKey, MetadataResponseTopic>::deserialize(
            version, bytes,
        );
        (
            header,
            MetadataResponse {
                throttle_time_ms,
                brokers,
                cluster_id,
                controller_id,
                topics,
            },
        )
    }
}

impl MetadataResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.cluster_id.is_none() && !_version >= 2 {
            return Err(SerializationError::NullValue(
                "cluster_id",
                _version,
                "MetadataResponse",
            ));
        }
        if self.throttle_time_ms != i32::default() && _version >= 3 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "throttle_time_ms",
                _version,
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.node_id.serialize(version, bytes)?;
        Ok(())
    }
}

impl MetadataResponseBrokerKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataResponseBrokerKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let node_id = i32::deserialize(version, bytes);
        MetadataResponseBrokerKey { node_id }
    }
}

impl ToBytes for MetadataResponseBroker {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.host.serialize(version, bytes)?;
        self.port.serialize(version, bytes)?;
        if version >= 1 {
            self.rack.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl MetadataResponseBroker {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.rack.is_none() && !_version >= 1 {
            return Err(SerializationError::NullValue(
                "rack",
                _version,
                "MetadataResponseBroker",
            ));
        }
        Ok(())
    }
}

impl FromBytes for MetadataResponseBroker {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let host = String::deserialize(version, bytes);
        let port = i32::deserialize(version, bytes);
        let rack = if version >= 1 {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        MetadataResponseBroker { host, port, rack }
    }
}

impl ToBytes for MetadataResponseTopicKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl MetadataResponseTopicKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataResponseTopicKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        MetadataResponseTopicKey { name }
    }
}

impl ToBytes for MetadataResponseTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, bytes)?;
        if version >= 1 {
            self.is_internal.serialize(version, bytes)?;
        }
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl MetadataResponseTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataResponseTopic {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let is_internal = if version >= 1 {
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, bytes)?;
        self.partition_index.serialize(version, bytes)?;
        self.leader_id.serialize(version, bytes)?;
        if version >= 7 {
            self.leader_epoch.serialize(version, bytes)?;
        }
        self.replica_nodes.serialize(version, bytes)?;
        self.isr_nodes.serialize(version, bytes)?;
        if version >= 5 {
            self.offline_replicas.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl MetadataResponsePartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for MetadataResponsePartition {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let partition_index = i32::deserialize(version, bytes);
        let leader_id = i32::deserialize(version, bytes);
        let leader_epoch = if version >= 7 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let replica_nodes = Vec::<i32>::deserialize(version, bytes);
        let isr_nodes = Vec::<i32>::deserialize(version, bytes);
        let offline_replicas = if version >= 5 {
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
