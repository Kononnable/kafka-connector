use crate::clients::consumer::consumer_loop::Assignments;
use crate::protocol_consts::{Broker, Epoch, Offset, Partition};
use std::collections::HashMap;
// TODO: pub(crate) or similar - internal use only(panics)

#[derive(Debug, Default)]
struct SubscribedPartition {
    // TODO: allow reads from non-leader
    leader_broker: Broker,
    next_offset_to_fetch: Option<Offset>,
    leader_epoch: Epoch,
}
#[derive(Debug, Default)]
struct SubscribedTopic {
    partitions: HashMap<Partition, SubscribedPartition>,
}
#[derive(Debug, Default)]
pub struct Subscriptions {
    inner: HashMap<String, SubscribedTopic>,
}
impl Subscriptions {
    pub fn add_partition(
        &mut self,
        topic: &str,
        partition: Partition,
        leader_broker: Broker,
        leader_epoch: Epoch,
    ) {
        self.inner
            .entry(topic.to_owned())
            .or_default()
            .partitions
            .insert(
                partition,
                SubscribedPartition {
                    leader_broker,
                    next_offset_to_fetch: None,
                    leader_epoch,
                },
            );
    }
    pub fn set_offset_for_partition(&mut self, topic: &str, partition: Partition, offset: Offset) {
        self.inner
            .entry(topic.to_owned())
            .or_default()
            .partitions
            .get_mut(&partition)
            .unwrap()
            .next_offset_to_fetch = Some(offset);
    }
    pub fn get_partitions_without_offset_by_broker(
        &self,
    ) -> HashMap<Broker, HashMap<String, Vec<(Partition, Epoch)>>> {
        let mut result: HashMap<Broker, HashMap<String, Vec<(Partition, Epoch)>>> = HashMap::new();
        for (topic_name, topic) in self.inner.iter() {
            for (partition_idx, partition) in topic
                .partitions
                .iter()
                .filter(|x| x.1.next_offset_to_fetch.is_none())
            {
                result
                    .entry(partition.leader_broker)
                    .or_default()
                    .entry(topic_name.clone())
                    .or_default()
                    .push((*partition_idx, partition.leader_epoch));
            }
        }
        result
    }

    pub fn into_assignments(self) -> Assignments {
        let mut result: Assignments = HashMap::new();
        for (topic_name, topic) in self.inner {
            for (partition_idx, partition) in topic.partitions {
                result
                    .entry(partition.leader_broker)
                    .or_default()
                    .entry(topic_name.clone())
                    .or_default()
                    .insert(
                        partition_idx,
                        (
                            partition.next_offset_to_fetch.unwrap(),
                            partition.leader_epoch,
                        ),
                    );
            }
        }
        result
    }
}
