use std::{collections::HashMap, sync::Arc};

use crate::{
    cluster::{cluster_loop::BrokerState, Cluster},
    consumer::consumer_loop::{ConsumerGroupMetadata, ConsumerGroupMetadataMember},
};

use self::{
    consumer_loop::ConsumerLoopSignal,
    error::{ConfirmError, MessageError, SubscribeError},
    options::ConsumerOptions,
};

use async_stream::try_stream;
use futures_util::stream::Stream;
use kafka_connector_protocol::{
    api::{
        find_coordinator::FindCoordinatorRequest,
        join_group::{JoinGroupRequest, JoinGroupRequestProtocols0},
    },
    custom_types::nullable_string::NullableString,
};
use log::{debug, trace};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

mod consumer_loop;
pub mod error;
pub mod options;

pub struct Consumer {
    cluster: Arc<Cluster>,
    options: ConsumerOptions,
    loop_signal_sender: UnboundedSender<ConsumerLoopSignal>,
    message_receiver: UnboundedReceiver<Vec<Message>>,
}

impl Consumer {
    pub async fn new(cluster: Arc<Cluster>, options: ConsumerOptions) -> Self {
        // TODO: remove unwraps
        let coordinator_response = cluster
            .send_request_to_any_broker(
                FindCoordinatorRequest {
                    key: options.group_id.clone(),
                    key_type: 0, // TODO: enum value?
                    tag_buffer: Default::default(),
                },
                None,
            )
            .await
            .unwrap();
        trace!("coordinator_response :{:#?}", coordinator_response);
        let mut brokers = cluster.inner.brokers.write().await;
        let coordinator_id = coordinator_response.node_id;
        let coordinator_broker = brokers.get_mut(&coordinator_response.node_id).unwrap();
        let broker = if let BrokerState::Alive { addr, broker } = coordinator_broker {
            broker
        } else {
            todo!()
        };
        let metadata = vec![
            0, 1, 0, 0, 0, 1, 0, 13, 112, 114, 111, 100, 117, 99, 101, 114, 95, 116, 101, 115, 116,
            0, 0, 0, 0, 0, 0, 0, 0,
        ]; // TODO:
        let join_group_response = broker
            .run_api_call_with_retry_raw(
                // TODO: Proper values
                JoinGroupRequest {
                    group_id: options.group_id.clone(),
                    session_timeout_ms: 10000,
                    rebalance_timeout_ms: 300000,
                    member_id: "".to_owned(),
                    group_instance_id: NullableString::None,
                    protocol_type: "consumer".to_owned(), // TODO: enum of sorts?
                    protocols: vec![
                        JoinGroupRequestProtocols0 {
                            name: "range".to_owned(), // TODO: enum of sorts?
                            metadata: metadata.clone(),
                            tag_buffer: Default::default(),
                        },
                        JoinGroupRequestProtocols0 {
                            name: "roundrobin".to_owned(), // TODO: enum of sorts?
                            metadata: metadata.clone(),
                            tag_buffer: Default::default(),
                        },
                    ],
                    tag_buffer: Default::default(),
                },
                None,
            )
            .await
            .err()
            .unwrap(); // TODO: Check error type
        trace!("join_group_response :{:#?}", join_group_response);

        let join_group_response2 = broker
            .run_api_call_with_retry_raw(
                // TODO: Proper values
                JoinGroupRequest {
                    group_id: options.group_id.clone(),
                    session_timeout_ms: 10000,
                    rebalance_timeout_ms: 300000,
                    member_id: join_group_response.1.member_id,
                    group_instance_id: NullableString::None,
                    protocol_type: "consumer".to_owned(), // TODO: enum of sorts?
                    protocols: vec![
                        JoinGroupRequestProtocols0 {
                            name: "range".to_owned(), // TODO: enum of sorts?
                            metadata: metadata.clone(),
                            tag_buffer: Default::default(),
                        },
                        JoinGroupRequestProtocols0 {
                            name: "roundrobin".to_owned(), // TODO: enum of sorts?
                            metadata: metadata.clone(),
                            tag_buffer: Default::default(),
                        },
                    ],
                    tag_buffer: Default::default(),
                },
                None,
            )
            .await
            .unwrap();
        trace!("join_group_response2 :{:#?}", join_group_response2);

        let consumer_group_metadata = ConsumerGroupMetadata {
            generation_id: join_group_response2.generation_id,
            protocol_type: join_group_response2.protocol_type.unwrap().into(),
            protocol_name: join_group_response2.protocol_name.into(),
            leader: join_group_response2.leader,
            member_id: join_group_response2.member_id,
            group_instances: join_group_response2
                .members
                .into_iter()
                .map(|x| {
                    let tmp: NullableString = x.group_instance_id.into();
                    (
                        tmp.into(),
                        ConsumerGroupMetadataMember {
                            member_id: x.member_id,
                        },
                    )
                })
                .fold(HashMap::new(), |mut a, b| {
                    a.entry(b.0).or_insert_with(Vec::new).push(b.1);
                    a
                }),
        };

        let (loop_signal_sender, loop_signal_receiver) = unbounded_channel();
        let (message_sender, message_receiver) = unbounded_channel();

        tokio::spawn(consumer_loop::consumer_loop(
            loop_signal_receiver,
            cluster.inner.clone(),
            consumer_group_metadata,
            options.clone(),
            coordinator_id,
            message_sender,
        ));

        drop(brokers);
        Consumer {
            cluster,
            options,
            loop_signal_sender,
            message_receiver,
        }
    }
    pub async fn stream(
        mut self,
    ) -> Result<impl Stream<Item = Result<Message, MessageError>>, SubscribeError> {
        // TODO: unwrap
        self.loop_signal_sender
            .send(ConsumerLoopSignal::Fetch)
            .unwrap();
        Ok(try_stream! {
            while let Some(message_batch) = self.message_receiver.recv().await {
                for message in message_batch{

                    yield message;
                }
            }

        })
    }
    /// e.g. before rebalance - store offset etc.
    pub fn on_partition_revoked() {
        todo!()
    }
    /// e.g. after rebalance - seek offset if needed etc.
    pub fn on_partition_assigned() {
        todo!()
    }
}

impl Drop for Consumer {
    fn drop(&mut self) {
        debug!("Consumer is being dropped");
        self.loop_signal_sender
            .send(ConsumerLoopSignal::Shutdown)
            .expect("Consumer loop should be alive.")
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    // TODO: Change to zero-copy solution
    pub topic: String,
    pub key: Vec<u8>,
    pub payload: Vec<u8>,
    pub partition: i32,
    pub offset: i64,
    pub timestamp: i64,
    pub headers: HashMap<String, Vec<u8>>,
}
impl Message {
    pub fn confirm(&self) -> Result<(), ConfirmError> {
        todo!()
    }
}
