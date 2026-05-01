use crate::clients::consumer::error::ConsumeError;
use crate::clients::consumer::error::ConsumeError::MetadataFetchFailed;
use crate::cluster::controller::{ClusterController, ForceRefresh};
use crate::cluster::error::ApiCallError;
use crate::protocol_consts::consumer_protocol_assignment::{
    ConsumerProtocolAssignment, TopicPartition,
};
use crate::protocol_consts::consumer_protocol_subscription::ConsumerProtocolSubscription;
use bytes::BytesMut;
use kafka_connector_protocol::join_group_response::JoinGroupResponseMember;
use kafka_connector_protocol::metadata_request::MetadataRequest;
use kafka_connector_protocol::{ApiError, ApiRequest, ApiVersion, FromBytes};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::error;

#[derive(Debug)]
pub struct RoundRobin {}

// impl ConsumerAssignmentStrategy for RoundRobin {
impl RoundRobin {
    pub fn name(&self) -> &'static str {
        "roundrobin"
    }

    pub fn subscription_userdata(&self) -> Option<Vec<u8>> {
        None
    }

    pub async fn assign_partitions(
        &self,
        members: Vec<JoinGroupResponseMember>,
        controller: &Arc<ClusterController>,
    ) -> Result<Vec<(String, ConsumerProtocolAssignment)>, ConsumeError> {
        // TODO: refresh metadata before calling an assigner - remove controller from parameters, and async + (result return type, add metadata as parameter)?
        let mut assignment: HashMap<String, HashMap<String, Vec<i32>>> = HashMap::new();
        let members = members
            .into_iter()
            .map(|x| {
                let mut metadata = BytesMut::from(x.metadata.as_slice());
                let metadata_version = i16::deserialize(ApiVersion(0), &mut metadata);
                let metadata = ConsumerProtocolSubscription::deserialize(
                    ApiVersion(metadata_version),
                    &mut metadata,
                );
                (x.member_id, metadata)
            })
            .collect::<Vec<_>>();
        let mut all_topics = members
            .iter()
            .flat_map(|x| x.1.topics.clone())
            .map(|x| (x, 0..0))
            .collect::<HashMap<_, _>>();

        for (topic, topic_metadata) in controller
            .get_metadata(all_topics.keys().cloned().collect(), ForceRefresh::Yes)
            .await
            .map_err(MetadataFetchFailed)?
            .into_iter()
        {
            if let Some(error_code) = topic_metadata.error_code {
                match error_code {
                    ApiError::UnknownTopicOrPartition => {
                        // TODO: ?
                    }
                    ApiError::InvalidTopicException => {
                        error!("{} is not a valid name for a Kafka Topic", topic);
                    }
                    _ => Err(ApiCallError::UnexpectedErrorCode(
                        MetadataRequest::get_api_key(),
                        error_code,
                        "topics.error_code",
                    ))?,
                }
            }

            *all_topics.get_mut(&topic).unwrap() = 0..topic_metadata.partitions.len();

            for partition_metadata in topic_metadata.partitions {
                if let Some(error_code) = partition_metadata.error_code {
                    Err(ConsumeError::ApiCallError(
                        ApiCallError::UnexpectedErrorCode(
                            MetadataRequest::get_api_key(),
                            error_code,
                            "topics.partitions.error_code",
                        ),
                    ))?
                }
            }
        }
        let mut sorted_topics: Vec<_> = all_topics.into_iter().collect();
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
                    .push(partition as i32);
            }
        }

        Ok(assignment
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
            .collect())
    }
}
