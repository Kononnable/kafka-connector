use std::collections::HashMap;

use kafka_connector_protocol::api::metadata::MetadataResponse0;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PartitionMetadata {
    pub leader_id: i32,
    pub leader_epoch: Option<i32>,
    pub replica_nodes: Vec<i32>,
    pub isr_nodes: Vec<i32>,
    pub offline_replicas: Option<Vec<i32>>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TopicMetadata {
    pub is_internal: bool,
    pub partitions: HashMap<i32, PartitionMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Metadata {
    pub brokers: HashMap<i32, (String, i32)>,
    pub topics: HashMap<String, TopicMetadata>,
}

impl From<MetadataResponse0> for Metadata {
    fn from(response: MetadataResponse0) -> Self {
        let mut brokers = HashMap::new();
        for broker in response.brokers {
            brokers.insert(broker.node_id, (broker.host, broker.port));
        }
        let mut topics = HashMap::new();
        for topic in response.topics {
            topics.insert(
                topic.name,
                TopicMetadata {
                    is_internal: topic.is_internal.unwrap_or_default(),
                    partitions: topic
                        .partitions
                        .into_iter()
                        .map(|partition| {
                            (
                                partition.partition_index,
                                PartitionMetadata {
                                    isr_nodes: partition.isr_nodes,
                                    leader_epoch: partition.leader_epoch,
                                    leader_id: partition.leader_id,
                                    offline_replicas: partition.offline_replicas,
                                    replica_nodes: partition.replica_nodes,
                                },
                            )
                        })
                        .collect(),
                },
            );
        }

        Metadata { brokers, topics }
    }
}
