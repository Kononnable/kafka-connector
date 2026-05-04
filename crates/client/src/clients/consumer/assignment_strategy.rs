use crate::protocol_consts::consumer_protocol_assignment::ConsumerProtocolAssignment;
use kafka_connector_protocol::join_group_response::JoinGroupResponseMember;
use std::fmt::Debug;

pub mod round_robin;

pub trait ConsumerAssignmentStrategy: Debug {
    fn name(&self) -> &'static str;

    // TODO: add parameter for consumer(?) ability to set userdata without knowledge of consumer/client state may be worthless
    fn subscription_userdata(&self) -> Option<Vec<u8>>;

    fn assign_partitions(
        &self,
        members: Vec<JoinGroupResponseMember>,
    ) -> Vec<(String, ConsumerProtocolAssignment)>;
}
