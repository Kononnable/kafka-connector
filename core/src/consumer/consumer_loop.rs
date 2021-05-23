use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::cluster::{cluster_loop::BrokerState, ClusterInner};

use kafka_connector_protocol::{
    api::{
        fetch::{FetchRequest, FetchRequestTopics0, FetchRequestTopicsPartitions0},
        heartbeat::HeartbeatRequest,
        leave_group::LeaveGroupRequest,
        list_offsets::{
            ListOffsetsRequest, ListOffsetsRequestTopics0, ListOffsetsRequestTopicsPartitions0,
        },
        sync_group::{SyncGroupRequest, SyncGroupRequestAssignments0},
    },
    custom_types::tag_buffer::TagBuffer,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use super::{options::ConsumerOptions, Message};

#[derive(Debug)]
pub(super) enum ConsumerLoopSignal {
    Fetch,
    /// Notify brokers of being alive
    Heartbeat,
    /// Disconnect from kafka brokers, clean up
    Shutdown,
}

#[derive(Debug, Clone)]
pub(super) struct ConsumerGroupMetadata {
    pub generation_id: i32,
    pub protocol_type: Option<String>,
    pub protocol_name: Option<String>,
    pub leader: String,
    pub member_id: String,
    pub group_instances: HashMap<Option<String>, Vec<ConsumerGroupMetadataMember>>,
}

impl ConsumerGroupMetadata {
    pub fn is_leader(&self) -> bool {
        self.leader == self.member_id
    }
}

#[derive(Debug, Clone)]
pub struct ConsumerGroupMetadataMember {
    pub member_id: String,
}

pub(super) async fn consumer_loop(
    loop_signal_receiver: UnboundedReceiver<ConsumerLoopSignal>,
    cluster: Arc<ClusterInner>,
    initial_metadata: ConsumerGroupMetadata,
    options: ConsumerOptions,
    coordinator_id: i32,
    message_sender: UnboundedSender<Vec<Message>>,
) {
    if initial_metadata.is_leader() {
        for group_instance in initial_metadata.group_instances.iter() {
            let request = SyncGroupRequest {
                group_id: options.group_id.clone(),
                generation_id: initial_metadata.generation_id,
                member_id: initial_metadata.member_id.clone(),
                group_instance_id: group_instance.0.clone().into(),
                protocol_type: initial_metadata.protocol_type.clone().into(),
                protocol_name: initial_metadata.protocol_name.clone().into(),
                assignments: group_instance
                    .1
                    .iter()
                    .map(|x| {
                        SyncGroupRequestAssignments0 {
                            assignment: vec![
                                0, // TODO:
                                0, 0, 0, 0, 1, 0, 13, 112, 114, 111, 100, 117, 99, 101, 114, 95,
                                116, 101, 115, 116, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                            ],
                            member_id: x.member_id.clone(),
                            tag_buffer: TagBuffer::default(),
                        }
                    })
                    .collect(),
                tag_buffer: TagBuffer {},
            };

            let mut brokers = cluster.brokers.write().await;
            let coordinator = brokers.get_mut(&coordinator_id).unwrap();
            let broker = if let BrokerState::Alive { addr, broker } = coordinator {
                broker
            } else {
                todo!()
            };
            broker
                .run_api_call_with_retry_raw(request, None)
                .await
                .unwrap();
        }
    } else {
        todo!()
    }
    let offsets = {
        // TODO:
        let mut brokers = cluster.brokers.write().await;
        let coordinator = brokers.get_mut(&coordinator_id).unwrap();
        let broker = if let BrokerState::Alive { addr, broker } = coordinator {
            broker
        } else {
            todo!()
        };
        let offsets = broker
            .run_api_call_with_retry(
                ListOffsetsRequest {
                    replica_id: -1,
                    isolation_level: 0,
                    topics: vec![ListOffsetsRequestTopics0 {
                        name: options.topics.first().unwrap().clone(),
                        partitions: vec![ListOffsetsRequestTopicsPartitions0 {
                            current_leader_epoch: 0,
                            partition_index: 0,
                            timestamp: -1,
                        }],
                    }],
                },
                None,
            )
            .await
            .unwrap();
        let errored = offsets
            .topics
            .iter()
            .find(|x| x.partitions.iter().find(|y| y.error_code != 0).is_some());
        if let Some(x) = errored {
            panic!("Error fetching offset data {:?}", x)
        }
        offsets
            .topics
            .into_iter()
            .map(|topic| {
                (
                    topic.name,
                    topic
                        .partitions
                        .into_iter()
                        .map(|partition| (partition.partition_index, partition.offset))
                        .collect(),
                )
            })
            .collect::<HashMap<String, HashMap<i32, Option<i64>>>>()
    };

    let send_heartbeat = futures::stream::repeat_with(|| ConsumerLoopSignal::Heartbeat)
        .throttle(Duration::from_secs(2)); // TODO: configurable,  change value

    let mut stream =
        Box::pin(send_heartbeat.merge(UnboundedReceiverStream::new(loop_signal_receiver)));

    while let Some(signal) = stream.next().await {
        match signal {
            ConsumerLoopSignal::Fetch => {
                let mut brokers = cluster.brokers.write().await;
                let coordinator = brokers.get_mut(&coordinator_id).unwrap();
                let broker = if let BrokerState::Alive { addr, broker } = coordinator {
                    broker
                } else {
                    todo!()
                };
                log::trace!("Sending fetch request");
                let fetch_response = broker
                    .run_api_call_with_retry(
                        FetchRequest {
                            replica_id: -1,
                            max_wait_ms: 500,
                            min_bytes: 1,
                            max_bytes: 52428800,
                            isolation_level: 1,
                            session_id: 0,
                            session_epoch: -1,
                            topics: vec![
                                // TODO:
                                FetchRequestTopics0 {
                                    topic: offsets.iter().next().unwrap().0.clone(),
                                    partitions: vec![FetchRequestTopicsPartitions0 {
                                        partition: *offsets
                                            .iter()
                                            .next()
                                            .unwrap()
                                            .1
                                            .iter()
                                            .next()
                                            .unwrap()
                                            .0,
                                        current_leader_epoch: -1,
                                        fetch_offset: offsets
                                            .iter()
                                            .next()
                                            .unwrap()
                                            .1
                                            .iter()
                                            .next()
                                            .unwrap()
                                            .1
                                            .unwrap_or_default(),
                                        last_fetched_epoch: 0,
                                        log_start_offset: -1,
                                        partition_max_bytes: 1048576,
                                        tag_buffer: TagBuffer::default(),
                                    }],
                                    tag_buffer: TagBuffer::default(),
                                },
                            ],
                            forgotten_topics_data: vec![],
                            rack_id: "".to_owned(),
                            tag_buffer: TagBuffer::default(),
                        },
                        None,
                    )
                    .await
                    .unwrap();
                log::trace!("Fetch response {:?}", fetch_response);
                let responses: Vec<Message> = fetch_response
                    .responses
                    .into_iter()
                    .flat_map(|topic| {
                        let topic_name = topic.topic.clone();
                        let v: Vec<_> = topic
                            .partition_responses
                            .into_iter()
                            .flat_map(move |partition| {
                                let partition_no = partition.partition;
                                if partition.error_code != 0 {
                                    panic!(
                                        "Error fetching partition data - error code:  {}",
                                        partition.error_code
                                    )
                                }
                                let topic_name = topic_name.clone();
                                let v: Vec<_> = partition
                                    .record_set
                                    .batches
                                    .into_iter()
                                    .flat_map(move |batch| {
                                        let v: Vec<_> = batch
                                            .records
                                            .into_iter()
                                            .map(|record| {
                                                let headers: Vec<_> = record.headers.into();
                                                Message {
                                                    topic: topic_name.clone(),
                                                    key: record.key.into(),
                                                    payload: record.value.into(),
                                                    partition: partition_no,
                                                    offset: record.offset,
                                                    timestamp: record.timestamp,
                                                    headers: headers
                                                        .into_iter()
                                                        .map(|header| {
                                                            (header.key.into(), header.value.into())
                                                        })
                                                        .collect(),
                                                }
                                            })
                                            .collect();
                                        v.into_iter()
                                    })
                                    .collect();
                                v.into_iter()
                            })
                            .collect();
                        v
                    })
                    .collect();
                log::trace!("Sending kafka messages to consumer {:?}", responses);
                message_sender.send(responses).unwrap();
                // TODO: If no messages schedule another fetch
            }
            ConsumerLoopSignal::Heartbeat => {
                let mut brokers = cluster.brokers.write().await;
                let coordinator = brokers.get_mut(&coordinator_id).unwrap();
                let broker = if let BrokerState::Alive { addr, broker } = coordinator {
                    broker
                } else {
                    todo!()
                };
                let x = initial_metadata
                    .group_instances
                    .iter()
                    .filter(|x| {
                        x.1.iter()
                            .any(|z| z.member_id == initial_metadata.member_id)
                    })
                    .map(|x| x.0)
                    .collect::<Vec<_>>();
                for group_instance_id in x {
                    broker
                        .run_api_call_with_retry(
                            HeartbeatRequest {
                                group_id: options.group_id.clone(),
                                generation_id: initial_metadata.generation_id,
                                member_id: initial_metadata.member_id.clone(),
                                group_instance_id: group_instance_id.clone().into(),
                                tag_buffer: TagBuffer::default(),
                            },
                            None,
                        )
                        .await
                        .unwrap();
                }
            }
            ConsumerLoopSignal::Shutdown => {
                let mut brokers = cluster.brokers.write().await;
                let broker = brokers.iter_mut().next().unwrap().1;
                let broker = if let BrokerState::Alive { addr, broker } = broker {
                    broker
                } else {
                    todo!()
                };

                broker
                    .run_api_call_with_retry(
                        LeaveGroupRequest {
                            group_id: options.group_id.clone(),
                            members: vec![],
                            tag_buffer: Default::default(),
                        },
                        None,
                    )
                    .await
                    .unwrap();

                break;
            }
        }
    }
    // TODO: Leave group gracefully if broker connection alive
    log::trace!("Consumer loop close")
}
