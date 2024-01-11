use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct MetadataResponse {
    pub throttle_time_ms: i32,
    pub brokers: Vec<MetadataResponseBroker>,
    pub cluster_id: String,
    pub controller_id: i32,
    pub topics: Vec<MetadataResponseTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataResponseBroker {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
    pub rack: String,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataResponseTopic {
    pub error_code: i16,
    pub name: String,
    pub is_internal: bool,
    pub partitions: Vec<MetadataResponsePartition>,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataResponsePartition {
    pub error_code: i16,
    pub partition_index: i32,
    pub leader_id: i32,
    pub leader_epoch: i32,
    pub replica_nodes: Vec<i32>,
    pub isr_nodes: Vec<i32>,
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
        let brokers = if version >= 0 {
            Vec::<MetadataResponseBroker>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let cluster_id = if version >= 2 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let controller_id = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<MetadataResponseTopic>::deserialize(version, bytes)
        } else {
            Default::default()
        };
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

impl FromBytes for MetadataResponseBroker {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let node_id = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let host = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let port = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let rack = if version >= 1 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        MetadataResponseBroker {
            node_id,
            host,
            port,
            rack,
        }
    }
}

impl FromBytes for MetadataResponseTopic {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let is_internal = if version >= 1 {
            bool::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<MetadataResponsePartition>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        MetadataResponseTopic {
            error_code,
            name,
            is_internal,
            partitions,
        }
    }
}

impl FromBytes for MetadataResponsePartition {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_id = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = if version >= 7 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let replica_nodes = if version >= 0 {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let isr_nodes = if version >= 0 {
            Vec::<i32>::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
