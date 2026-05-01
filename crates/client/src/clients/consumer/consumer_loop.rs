use crate::clients::consumer::error::ConsumeError;
use crate::clients::consumer::error::ConsumeError::MetadataFetchFailed;
use crate::clients::consumer::options::KafkaConsumerOptions;
use crate::clients::consumer::record::Record;
use crate::clients::consumer::subscribed_partitions::Subscriptions;
use crate::cluster::controller::{ClusterController, ForceRefresh};
use crate::cluster::error::ApiCallError;
use crate::protocol_consts::consumer_protocol_assignment::ConsumerProtocolAssignment;
use crate::protocol_consts::consumer_protocol_subscription::ConsumerProtocolSubscription;
use crate::protocol_consts::{FindCoordinatorKeyType, IsolationLevel};
use bytes::BytesMut;
use futures::future::{join_all, select_all};
use kafka_connector_protocol::fetch_request::{FetchPartition, FetchRequest, FetchableTopic};
use kafka_connector_protocol::fetch_response::FetchResponse;
use kafka_connector_protocol::find_coordinator_request::FindCoordinatorRequest;
use kafka_connector_protocol::join_group_request::{
    JoinGroupRequest, JoinGroupRequestProtocol, JoinGroupRequestProtocolKey,
};
use kafka_connector_protocol::list_offset_request::{
    ListOffsetPartition, ListOffsetRequest, ListOffsetTopic,
};
use kafka_connector_protocol::metadata_request::MetadataRequest;
use kafka_connector_protocol::offset_fetch_request::{OffsetFetchRequest, OffsetFetchRequestTopic};
use kafka_connector_protocol::records::record_batch::RecordBatch;
use kafka_connector_protocol::sync_group_request::{SyncGroupRequest, SyncGroupRequestAssignment};
use kafka_connector_protocol::{ApiError, ApiRequest, ApiVersion, FromBytes, ToBytes};
use std::collections::HashMap;
use std::ops::Add;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, instrument, warn};

// TODO: remove, migrate to SUbscriberPartitions
/// Broker -> Topic -> Partition -> (next_offset_to_fetch, current_leader_epoch)
pub type Assignments = HashMap<i32, HashMap<String, HashMap<i32, (i64, i32)>>>;

#[derive(Debug)]
enum ConsumerLoopState {
    Initializing,
    Consuming,
}

pub struct ConsumerLoop {
    controller: Arc<ClusterController>,
    consumer_options: KafkaConsumerOptions,
    record_sender: mpsc::Sender<Record>,
    /// Broker -> Topic -> Partition -> (next_offset_to_fetch, current_leader_epoch)
    partitions_to_consume: Assignments,
    deserialization_buffer: BytesMut,
    group_coordinator: Option<i32>,
    state: ConsumerLoopState,
    member_id: String,
}

impl ConsumerLoop {
    #[instrument(level = "debug", skip_all)]
    pub async fn start(
        controller: Arc<ClusterController>,
        consumer_options: KafkaConsumerOptions,
        record_sender: mpsc::Sender<Record>,
        command_receiver: mpsc::Receiver<()>,
    ) {
        ConsumerLoop {
            controller,
            consumer_options,
            record_sender,
            partitions_to_consume: Default::default(),
            deserialization_buffer: BytesMut::new(),
            group_coordinator: None,
            state: ConsumerLoopState::Initializing,
            member_id: "".to_owned(),
        }
        .run(command_receiver)
        .await;
    }

    #[instrument(level = "debug", skip(self))]
    async fn run(self, mut command_receiver: mpsc::Receiver<()>) {
        debug!("Consumer loop started.");

        let mut main_loop = Box::pin(self.main_loop());
        tokio::select! {
            biased;
            None = command_receiver.recv() => {}
            _  = &mut main_loop => {}
        }

        debug!("Consumer loop is closing");
    }

    #[instrument(level = "debug", skip(self))]
    async fn main_loop(mut self) {
        let mut fetch_requests_in_flight = None;
        loop {
            // TODO: merge into select, self.state, fetch_request_in_flight.is_some() as condition on branch
            // TODO: add heartbeat support
            // TODO: error handling, currently error in any state will transfer to Initializing which is not always required
            // e.g. partition leader changes
            match self.state {
                ConsumerLoopState::Initializing => {
                    if let Err(err) = self.get_topic_assignments().await {
                        let event_msg = format!(
                            "Encountered {err} during consumer metadata synchronization, retrying in 1000 ms."
                        );
                        if err.is_transisient() {
                            debug!(event_msg);
                        } else {
                            warn!(event_msg);
                        }
                        tokio::time::sleep(Duration::from_millis(1_000)).await;
                        continue;
                    }

                    self.state = ConsumerLoopState::Consuming;
                    fetch_requests_in_flight = Some(Box::pin(select_all(
                        self.partitions_to_consume
                            .keys()
                            .map(|broker_id| self.fetch_data(*broker_id))
                            .collect::<Vec<_>>(),
                    )));
                }
                ConsumerLoopState::Consuming => {
                    if fetch_requests_in_flight.is_none() {
                        // No partitions assigned
                        tokio::time::sleep(Duration::from_millis(50)).await;
                        continue;
                    };
                    let ((broker_id, response), _, in_progress) =
                        fetch_requests_in_flight.take().unwrap().await;
                    self.process_fetch_response(broker_id, response).await;

                    // Manually pool all remaining futures before scheduling new ones - makes consumption from multiple brokers fair
                    let mut new_in_progress = vec![self.fetch_data(broker_id)];
                    for mut future in in_progress.into_iter() {
                        let pool_result = future
                            .as_mut()
                            .poll(&mut Context::from_waker(Waker::noop()));
                        match pool_result {
                            Poll::Ready((broker_id, response)) => {
                                self.process_fetch_response(broker_id, response).await;
                                new_in_progress.push(self.fetch_data(broker_id));
                            }
                            Poll::Pending => {
                                new_in_progress.push(future);
                            }
                        }
                    }
                    fetch_requests_in_flight = Some(Box::pin(select_all(new_in_progress)));
                }
            }
        }
    }

    #[instrument(level = "debug", skip(self))]
    async fn get_topic_assignments(&mut self) -> Result<(), ConsumeError> {
        if self.group_coordinator.is_none() && self.consumer_options.group_id.is_some() {
            self.find_group_coordinator().await?;
        }

        let assignment = match self.consumer_options.group_id {
            None => None,
            Some(_) => self.join_and_sync_consumer_group().await?,
        };
        self.get_next_offsets_to_consume(assignment).await
    }
    async fn find_group_coordinator(&mut self) -> Result<(), ConsumeError> {
        let request = FindCoordinatorRequest {
            key: self.consumer_options.group_id.clone().unwrap(),
            key_type: FindCoordinatorKeyType::Group.into(),
        };
        let response = self.controller.make_api_call(None, request, None).await?;

        if let Some(error_code) = response.error_code {
            match error_code {
                ApiError::CoordinatorLoadInProgress | ApiError::CoordinatorNotAvailable => {
                    Err(ConsumeError::CoordinatorNotReady(error_code))?;
                }
                ApiError::GroupAuthorizationFailed => {
                    Err(ConsumeError::GroupAuthorizationFailed {
                        group_id: self.consumer_options.group_id.clone().unwrap(),
                        error: response.error_message,
                    })?
                }
                _ => Err(ApiCallError::UnexpectedErrorCode(
                    FindCoordinatorRequest::get_api_key(),
                    error_code,
                    "error_code",
                ))?,
            }
        }
        self.group_coordinator = Some(response.node_id);
        Ok(())
    }

    async fn join_and_sync_consumer_group(
        &mut self,
    ) -> Result<Option<ConsumerProtocolAssignment>, ConsumeError> {
        let group_id = self.consumer_options.group_id.as_ref().unwrap();
        let protocols = self
            .consumer_options
            .assignment_strategies
            .iter()
            .map(|strategy| {
                let subscription = ConsumerProtocolSubscription {
                    topics: self.consumer_options.topics.iter().cloned().collect(),
                    user_data: strategy.subscription_userdata(),
                    owned_partitions: vec![], // TODO: Sticky partitioner
                };
                let mut subscription_bytes = BytesMut::new(); // TODO: reuse some buffer(?)
                let max_supported_version =
                    ConsumerProtocolSubscription::get_max_supported_version();
                max_supported_version.serialize(ApiVersion(0), &mut subscription_bytes); // Version
                subscription
                    .serialize(max_supported_version, &mut subscription_bytes)
                    .unwrap();
                (
                    JoinGroupRequestProtocolKey {
                        name: strategy.name().to_owned(),
                    },
                    JoinGroupRequestProtocol {
                        metadata: subscription_bytes.to_vec(),
                    },
                )
            })
            .collect();

        let mut request = JoinGroupRequest {
            group_id: group_id.clone(),
            session_timeout_ms: self.consumer_options.session_timeout.as_millis() as i32,
            rebalance_timeout_ms: self.consumer_options.rebalance_timeout.as_millis() as i32,
            member_id: self.member_id.clone(),
            protocol_type: "consumer".to_string(),
            protocols,
        };
        let mut response = self
            .controller
            .make_api_call(None, request.clone(), None)
            .await?;
        if let Some(ApiError::MemberIdRequired) = response.error_code {
            // First time joining
            request.member_id = response.member_id;
            response = self.controller.make_api_call(None, request, None).await?;
        }

        if let Some(error_code) = response.error_code {
            match error_code {
                // TODO:
                _ => Err(ApiCallError::UnexpectedErrorCode(
                    JoinGroupRequest::get_api_key(),
                    error_code,
                    "error_code",
                ))?,
            }
        }
        let generation_id = response.generation_id;
        self.member_id = response.member_id;

        let assignments = if self.member_id == response.leader {
            let strategy = self
                .consumer_options
                .assignment_strategies
                .iter()
                .find(|strategy| strategy.name() == response.protocol_name)
                .unwrap(); // TODO: unwrap?
            let assignments = strategy
                .assign_partitions(response.members, &self.controller)
                .await?;

            assignments
                .into_iter()
                .map(|(member_id, assignment)| {
                    let mut assignment_bytes = BytesMut::new(); // TODO: reuse buffer
                    let max_supported_version =
                        ConsumerProtocolAssignment::get_max_supported_version();
                    max_supported_version.serialize(ApiVersion(0), &mut assignment_bytes); // Version
                    assignment
                        .serialize(max_supported_version, &mut assignment_bytes)
                        .unwrap();
                    SyncGroupRequestAssignment {
                        member_id,
                        assignment: assignment_bytes.to_vec(),
                    }
                })
                .collect()
        } else {
            vec![]
        };

        let request = SyncGroupRequest {
            group_id: group_id.clone(),
            generation_id,
            member_id: self.member_id.clone(),
            assignments,
        };
        let response = self.controller.make_api_call(None, request, None).await?;
        if let Some(error_code) = response.error_code {
            match error_code {
                // TODO:
                _ => Err(ApiCallError::UnexpectedErrorCode(
                    SyncGroupRequest::get_api_key(),
                    error_code,
                    "error_code",
                ))?,
            }
        }

        let mut assignment = BytesMut::from(response.assignment.as_slice());
        let assignment_version = i16::deserialize(ApiVersion(0), &mut assignment);
        let assignment = ConsumerProtocolAssignment::deserialize(
            ApiVersion(assignment_version),
            &mut assignment,
        );

        Ok(Some(assignment))
    }

    async fn get_next_offsets_to_consume(
        &mut self,
        assignment: Option<ConsumerProtocolAssignment>,
    ) -> Result<(), ConsumeError> {
        // TODO: will clear work for non-group consumer(?) maybe need to use temporary copy
        self.partitions_to_consume.clear();

        let topics = match &assignment {
            None => self.consumer_options.topics.clone(),
            Some(assignment) => assignment
                .assigned_partitions
                .iter()
                .map(|x| x.topic.to_owned())
                .collect(),
        };

        let mut partitions_to_consume = Subscriptions::default();

        // TODO: force refresh: no, but check assignments if all partitions from assignments are in the metadata response
        // if not, refresh with force:yes
        for (topic, topic_metadata) in self
            .controller
            .get_metadata(topics, ForceRefresh::Yes)
            .await
            .map_err(MetadataFetchFailed)?
            .into_iter()
        {
            if let Some(error_code) = topic_metadata.error_code {
                match error_code {
                    ApiError::UnknownTopicOrPartition => {
                        // TODO: is it still needed?
                        // self.partitions_to_consume.clear() is called
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

                // Take all partitions for consumer without a group, take only assigned partitions for consumer with a group
                if let Some(assignment) = &assignment
                    && assignment.assigned_partitions.iter().all(|x| {
                        x.topic != topic
                            || !x.partitions.contains(&partition_metadata.partition_index)
                    })
                {
                    continue;
                }
                partitions_to_consume.add_partition(
                    &topic,
                    partition_metadata.partition_index,
                    partition_metadata.leader_id,
                    partition_metadata.leader_epoch,
                );
            }
        }
        if assignment.is_none()
            && partitions_to_consume
                .get_partitions_without_offset_by_broker()
                .is_empty()
        {
            return Err(ConsumeError::NoValidTopicsFound());
        }

        if assignment.is_some() {
            // Fetch consumer group offsets

            let list_offset_responses = join_all(self.partitions_to_consume.iter().map(
                |(broker_id, assignments)| {
                    let request = OffsetFetchRequest {
                        group_id: self.consumer_options.group_id.clone().unwrap(),
                        topics: Some(
                            assignments
                                .iter()
                                .map(|(topic, partitions)| OffsetFetchRequestTopic {
                                    name: topic.to_owned(),
                                    partition_indexes: partitions.keys().map(|x| *x).collect(),
                                })
                                .collect(),
                        ),
                    };

                    let controller = self.controller.clone();
                    async move { controller.make_api_call(*broker_id, request, None).await }
                },
            ))
            .await;

            for response in list_offset_responses.into_iter() {
                for topic in response?.topics {
                    for partition in topic.partitions {
                        if let Some(error_code) = partition.error_code {
                            match error_code {
                                ApiError::FencedLeaderEpoch => {
                                    // partition leader was fenced during topic assignment, retrying
                                    Err(ConsumeError::PartitionLeaderFenced(
                                        topic.name.clone(),
                                        partition.partition_index,
                                    ))?
                                }
                                _ => Err(ApiCallError::UnexpectedErrorCode(
                                    ListOffsetRequest::get_api_key(),
                                    error_code,
                                    "topics.partitions.error_code",
                                ))?,
                            }
                        }
                        if partition.committed_offset >= 0 {
                            // -1 offset not commited
                            partitions_to_consume.set_offset_for_partition(
                                &topic.name,
                                partition.partition_index,
                                partition.committed_offset,
                            );
                        }
                    }
                }
            }
        }

        // Fetch offsets of partitions without consumer group commited offset
        let partitions_toc = partitions_to_consume.get_partitions_without_offset_by_broker();
        if !partitions_toc.is_empty() {
            let list_offset_responses =
                join_all(partitions_toc.into_iter().map(|(broker_id, assignments)| {
                    let request = ListOffsetRequest {
                        replica_id: -1,
                        isolation_level: IsolationLevel::ReadCommited.into(),
                        topics: assignments
                            .into_iter()
                            .map(|(topic, partitions)| ListOffsetTopic {
                                name: topic.to_owned(),
                                partitions: partitions
                                    .into_iter()
                                    .map(|(partition_index, current_leader_epoch)| {
                                        ListOffsetPartition {
                                            partition_index,
                                            current_leader_epoch,
                                            timestamp: self.consumer_options.offset_reset.into(),
                                            max_num_offsets: 0,
                                        }
                                    })
                                    .collect(),
                            })
                            .collect(),
                    };

                    let controller = self.controller.clone();
                    async move { controller.make_api_call(broker_id, request, None).await }
                }))
                .await;
            for response in list_offset_responses {
                for topic in response?.topics {
                    for partition in topic.partitions {
                        if let Some(error_code) = partition.error_code {
                            match error_code {
                                ApiError::FencedLeaderEpoch => {
                                    // partition leader was fenced during topic assignment, retrying
                                    Err(ConsumeError::PartitionLeaderFenced(
                                        topic.name.clone(),
                                        partition.partition_index,
                                    ))?
                                }
                                _ => Err(ApiCallError::UnexpectedErrorCode(
                                    ListOffsetRequest::get_api_key(),
                                    error_code,
                                    "topics.partitions.error_code",
                                ))?,
                            }
                        }
                        partitions_to_consume.set_offset_for_partition(
                            &topic.name,
                            partition.partition_index,
                            partition.offset,
                        );
                    }
                }
            }
        }

        self.partitions_to_consume = partitions_to_consume.to_assignments();
        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    fn fetch_data(
        &self,
        broker_id: i32,
    ) -> Pin<Box<impl Future<Output = (i32, Result<FetchResponse, ApiCallError>)> + use<>>> {
        let assignments = self.partitions_to_consume.get(&broker_id).unwrap();
        let request = FetchRequest {
            replica_id: -1,
            max_wait: self.consumer_options.max_wait.as_millis() as i32,
            min_bytes: self.consumer_options.min_bytes,
            max_bytes: self.consumer_options.max_bytes,
            isolation_level: IsolationLevel::ReadCommited.into(),
            session_id: 0,
            epoch: -1,
            topics: assignments
                .iter()
                .map(|(topic, partitions)| FetchableTopic {
                    name: topic.to_owned(),
                    fetch_partitions: partitions
                        .iter()
                        .map(|(partition_index, &(offset, epoch))| FetchPartition {
                            partition_index: *partition_index,
                            current_leader_epoch: epoch,
                            fetch_offset: offset,
                            log_start_offset: -1,
                            max_bytes: self.consumer_options.max_bytes_per_partition,
                        })
                        .collect(),
                })
                .collect(),
            forgotten: vec![],
        };

        let controller = self.controller.clone();
        Box::pin(async move {
            (
                broker_id,
                controller.make_api_call(broker_id, request, None).await,
            )
        })
    }

    #[instrument(level = "debug", skip(self, response))]
    async fn process_fetch_response(
        &mut self,
        broker: i32,
        response: Result<FetchResponse, ApiCallError>,
    ) {
        let Ok(response) = response else {
            info!(
                "Encountered {} during records consumption.",
                response.err().unwrap()
            );
            return;
        };
        if let Some(error_code) = response.error_code {
            let err = ApiCallError::UnexpectedErrorCode(
                FetchRequest::get_api_key(),
                error_code,
                "error_code",
            );
            info!("Encountered {err} during records consumption.");
            self.state = ConsumerLoopState::Initializing;
        }
        for topic_response in response.topics {
            let topic = topic_response.name.as_str();
            for partition_response in topic_response.partitions {
                if let Some(error_code) = response.error_code {
                    match error_code {
                        ApiError::UnknownTopicOrPartition
                        | ApiError::NotLeaderOrFollower
                        | ApiError::ReplicaNotAvailable
                        | ApiError::UnknownLeaderEpoch => {
                            self.state = ConsumerLoopState::Initializing;
                        }
                        ApiError::OffsetOutOfRange => {
                            self.state = ConsumerLoopState::Initializing;
                            self.partitions_to_consume
                                .get_mut(&broker)
                                .unwrap()
                                .get_mut(topic)
                                .unwrap()
                                .get_mut(&partition_response.partition_index)
                                .unwrap()
                                .0 = -1;
                        }
                        _ => {
                            let err = ApiCallError::UnexpectedErrorCode(
                                FetchRequest::get_api_key(),
                                error_code,
                                "topics.partitions.error_code",
                            );
                            info!("Encountered {err} during records consumption.");
                            self.state = ConsumerLoopState::Initializing;
                        }
                    }
                }
                if let Some(records) = partition_response.records {
                    let partition = partition_response.partition_index;
                    self.process_partition_records(broker, topic, partition, records)
                        .await;
                }
            }
        }
    }

    #[instrument(level = "debug", skip(self, records))]
    async fn process_partition_records(
        &mut self,
        broker: i32,
        topic: &str,
        partition: i32,
        records: Vec<u8>,
    ) {
        let mut last_processed_offset = None;
        self.deserialization_buffer
            .extend_from_slice(records.as_slice());

        while !self.deserialization_buffer.is_empty() {
            let batch = RecordBatch::decode(&mut self.deserialization_buffer);

            for record in batch.records {
                // TODO: Filter messages with lower offset than requested
                //       If we're fetching message with offset that is in the middle of the batch broker will send whole batch
                //       we have to filter out messages that have already been processed (e.g. before rebalancing)
                let offset = batch.base_offset + record.offset_delta.0 as i64;
                last_processed_offset = Some(offset);

                let record = Record {
                    timestamp: batch
                        .base_timestamp
                        .add(Duration::from_millis(record.timestamp_delta.0 as u64)),
                    key: record.key.0,
                    value: record.value.0,
                    topic: topic.to_owned(),
                    headers: record
                        .headers
                        .0
                        .into_iter()
                        .map(|header| (header.key.0, header.value.0))
                        .collect(),
                    partition,
                    offset,
                };

                // TODO: change mechanism so it does not hang here if buffer is full
                //       or explicitly block here, so we know exactly which messages were processed by consumer
                //       for auto commit offsets and rebalance
                // TODO: support for heartbeats and rebalances triggered by heartbeat response
                //      can this happen mid batch, or should batch be fully processed
                let _ = self.record_sender.send(record).await;
            }
        }
        if let Some(offset) = last_processed_offset {
            self.partitions_to_consume
                .get_mut(&broker)
                .unwrap()
                .get_mut(topic)
                .unwrap()
                .get_mut(&partition)
                .unwrap()
                .0 = offset + 1;
        }
    }
}
