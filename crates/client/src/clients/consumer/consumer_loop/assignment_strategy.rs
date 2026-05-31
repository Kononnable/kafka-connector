use crate::protocol_consts::consumer_protocol_assignment::ConsumerProtocolAssignment;
use crate::protocol_consts::consumer_protocol_subscription::ConsumerProtocolSubscription;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod round_robin;

pub trait ConsumerAssignmentStrategy: Debug + Send + Sync {
    fn name(&self) -> &'static str;

    // TODO: add parameter for consumer(?) ability to set userdata without knowledge of consumer/client state may be worthless
    fn subscription_userdata(&self) -> Option<Vec<u8>>;

    fn assign_partitions(
        &self,
        members: Vec<(String, ConsumerProtocolSubscription)>,
        topics_config: HashMap<String, i32>,
    ) -> Vec<(String, ConsumerProtocolAssignment)>;
}
