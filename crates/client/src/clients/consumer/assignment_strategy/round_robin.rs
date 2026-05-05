use crate::clients::consumer::assignment_strategy::ConsumerAssignmentStrategy;
use crate::protocol_consts::consumer_protocol_assignment::{
    ConsumerProtocolAssignment, TopicPartition,
};
use crate::protocol_consts::consumer_protocol_subscription::ConsumerProtocolSubscription;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RoundRobin {}

impl ConsumerAssignmentStrategy for RoundRobin {
    fn name(&self) -> &'static str {
        "roundrobin"
    }

    fn subscription_userdata(&self) -> Option<Vec<u8>> {
        None
    }

    fn assign_partitions(
        &self,
        members: Vec<(String, ConsumerProtocolSubscription)>,
        topics_config: HashMap<String, i32>,
    ) -> Vec<(String, ConsumerProtocolAssignment)> {
        let mut assignment: HashMap<String, HashMap<String, Vec<i32>>> = HashMap::new();

        let mut sorted_topics = topics_config
            .into_iter()
            .map(|(x, y)| (x, 0..y))
            .collect::<Vec<_>>();
        sorted_topics.sort_by_key(|x| x.0.clone()); // TODO: Clone

        let mut members_iter = members.into_iter().cycle();

        for (topic, partitions) in sorted_topics.into_iter() {
            for partition in partitions {
                let ass = members_iter
                    .find(|x| x.1.topics.contains(&topic))
                    .unwrap()
                    .0;
                assignment
                    .entry(ass)
                    .or_default()
                    .entry(topic.clone())
                    .or_default()
                    .push(partition);
            }
        }

        assignment
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    ConsumerProtocolAssignment {
                        assigned_partitions: v
                            .into_iter()
                            .map(|(topic, partitions)| TopicPartition { topic, partitions })
                            .collect(),
                        user_data: None,
                    },
                )
            })
            .collect()
    }
}
