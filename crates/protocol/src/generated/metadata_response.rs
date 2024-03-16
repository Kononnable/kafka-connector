use super::super::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each broker in the response.
    pub brokers: BTreeMap<MetadataResponseBrokerKey, MetadataResponseBroker>,

    /// The cluster ID that responding broker belongs to.
    pub cluster_id: Option<String>,

    /// The ID of the controller broker.
    pub controller_id: i32,

    /// Each topic in the response.
    pub topics: BTreeMap<MetadataResponseTopicKey, MetadataResponseTopic>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Ord, PartialOrd)]
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

#[derive(Clone, Debug, PartialEq, Default, Eq, Ord, PartialOrd)]
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
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 3 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let brokers = BTreeMap::<MetadataResponseBrokerKey, MetadataResponseBroker>::deserialize(
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
        let topics = BTreeMap::<MetadataResponseTopicKey, MetadataResponseTopic>::deserialize(
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

impl FromBytes for MetadataResponseBrokerKey {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let node_id = i32::deserialize(version, bytes);
        MetadataResponseBrokerKey { node_id }
    }
}

impl FromBytes for MetadataResponseBroker {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
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

impl FromBytes for MetadataResponseTopicKey {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        MetadataResponseTopicKey { name }
    }
}

impl FromBytes for MetadataResponseTopic {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
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

impl FromBytes for MetadataResponsePartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
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
