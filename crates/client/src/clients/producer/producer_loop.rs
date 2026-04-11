use crate::clients::producer::client::RecordAppend;
use crate::clients::producer::error::ProduceError;
use crate::clients::producer::future_record::FutureRecord;
use crate::clients::producer::options::{Acks, KafkaProducerOptions};
use crate::clients::producer::partitioner::Partitioner;
use crate::cluster::controller::{ClusterController, ForceRefresh};
use crate::cluster::error::ApiCallError;
use bytes::BytesMut;
use futures::future::{Either, select_all};
use kafka_connector_protocol::metadata_request::MetadataRequest;
use kafka_connector_protocol::metadata_response::{
    MetadataResponsePartition, MetadataResponseTopic,
};
use kafka_connector_protocol::produce_request::{
    PartitionProduceData, ProduceRequest, TopicProduceData,
};
use kafka_connector_protocol::produce_response::ProduceResponse;
use kafka_connector_protocol::records::base_types::{
    VarIntBytes, VarIntString, VarIntVec, VarLong,
};
use kafka_connector_protocol::records::header::Header;
use kafka_connector_protocol::records::record::Record;
use kafka_connector_protocol::records::record_batch::RecordBatch;
use kafka_connector_protocol::{ApiError, ApiRequest, ApiResponse};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::future;
use std::ops::Add;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, instrument};

pub(super) type ProduceRequestMessage = (FutureRecord, RecordAppendTx);
pub(super) type RecordAppendTx = oneshot::Sender<Result<RecordAppend, ProduceError>>;

struct ProduceApiCallResult {
    broker_id: i32,
    msg_id: u32,
    response: Result<Option<ProduceResponse>, ApiCallError>,
}

#[derive(Debug, Eq, PartialEq)]
enum BatchBuilderState {
    UnderConstruction { bytes_left: u32 },
    Full,
}

#[derive(Debug)]
struct BatchBuilder {
    state: BatchBuilderState,
    records: Vec<(Record, RecordAppend, RecordAppendTx)>,
}
impl BatchBuilder {
    pub fn new(bytes_left: u32) -> Self {
        BatchBuilder {
            state: BatchBuilderState::UnderConstruction { bytes_left },
            records: vec![],
        }
    }
}
#[derive(Default, Debug)]
struct Batch {
    records: Vec<(RecordAppend, RecordAppendTx)>,
}

#[derive(Debug)]
struct BrokerState<P>
where
    P: Partitioner,
{
    linger: Option<tokio::time::Instant>,
    options: Arc<KafkaProducerOptions<P>>,
    records_waiting: BTreeMap<(String, i32), VecDeque<BatchBuilder>>,
    active_calls: HashMap<u32, HashMap<String, HashMap<i32, Batch>>>,
}

impl<P> BrokerState<P>
where
    P: Partitioner,
{
    pub fn new(options: Arc<KafkaProducerOptions<P>>) -> BrokerState<P> {
        BrokerState {
            linger: None,
            options,
            records_waiting: Default::default(),
            active_calls: Default::default(),
        }
    }
    pub fn add_record(
        &mut self,
        topic: String,
        produce_record: Record,
        record_append: RecordAppend,
        tx: RecordAppendTx,
    ) {
        if self.linger.is_none() {
            self.linger = Some(tokio::time::Instant::now() + self.options.linger)
        }

        let partition_batches = self
            .records_waiting
            .entry((topic, record_append.partition))
            .or_default();

        let record_estimated_size = produce_record.estimate_size();
        let batch_builder = match partition_batches.back_mut() {
            Some(builder) => match builder.state {
                BatchBuilderState::UnderConstruction { bytes_left } => {
                    if bytes_left < record_estimated_size {
                        builder.state = BatchBuilderState::Full;
                        partition_batches
                            .push_back(BatchBuilder::new(self.options.batch_size_bytes));
                        partition_batches.back_mut().unwrap()
                    } else {
                        builder
                    }
                }
                BatchBuilderState::Full => {
                    partition_batches.push_back(BatchBuilder::new(self.options.batch_size_bytes));
                    partition_batches.back_mut().unwrap()
                }
            },
            None => {
                partition_batches.push_back(BatchBuilder::new(self.options.batch_size_bytes));
                partition_batches.back_mut().unwrap()
            }
        };

        batch_builder
            .records
            .push((produce_record, record_append, tx));
        batch_builder.state = if let BatchBuilderState::UnderConstruction { bytes_left } =
            batch_builder.state
            && bytes_left > record_estimated_size
        {
            BatchBuilderState::UnderConstruction {
                bytes_left: bytes_left - record_estimated_size,
            }
        } else {
            BatchBuilderState::Full
        };
    }
}

pub struct ProducerLoop<P>
where
    P: Partitioner,
{
    controller: Arc<ClusterController>,
    producer_options: Arc<KafkaProducerOptions<P>>,
    serialization_buffer: BytesMut,
    brokers: HashMap<i32, BrokerState<P>>,
    /// Internal counter to provide unique identifiers to track messages in flight
    produce_calls_counter: u32,
}

// TODO: add max_request_size support
// TODO: add multiple requests in flight support (more than one batch with same broker, topic, partition)
// TODO: Produce call error handling
impl<P> ProducerLoop<P>
where
    P: Partitioner,
{
    #[instrument(level = "debug", skip_all)]
    pub async fn start(
        controller: Arc<ClusterController>,
        producer_options: KafkaProducerOptions<P>,
        receiver: mpsc::UnboundedReceiver<ProduceRequestMessage>,
    ) {
        let serialization_buffer = BytesMut::with_capacity(controller.options.advanced.buffer_size);
        let producer_options = Arc::new(producer_options);
        ProducerLoop {
            controller,
            producer_options,
            serialization_buffer,
            brokers: HashMap::new(),
            produce_calls_counter: 0,
        }
        .run(receiver)
        .await;
    }

    #[instrument(level = "debug", skip_all)]
    async fn run(mut self, mut receiver: mpsc::UnboundedReceiver<ProduceRequestMessage>) {
        debug!("Producer loop started.");

        let mut send_message_task = select_all([self.send_produce_request(-1)]);

        loop {
            tokio::select! {
                signal = receiver.recv() => {
                    match signal {
                        Some(request) => {
                            self.add_message(request).await;

                        }
                        None => { break;}
                    }
                },
                (response, _, api_calls_in_flight) = &mut send_message_task => {
                    self.on_produce_response(response).await;
                    if !api_calls_in_flight.is_empty() {
                        send_message_task = select_all(api_calls_in_flight);
                    }

                },
                broker_id = self.ready_to_send_data() => {
                    let mut futures = send_message_task.into_inner();
                    futures.push(self.send_produce_request(broker_id));
                    send_message_task = select_all(futures);
                }
            }
        }

        self.brokers.into_iter().for_each(|(_, broker_data)| {
            broker_data
                .records_waiting
                .into_iter()
                .flat_map(|(_, batch_data)| batch_data)
                .flat_map(|x| x.records)
                .for_each(|(_, _, tx)| {
                    let _ = tx.send(Err(ProduceError::ProducerClosed));
                });
            broker_data
                .active_calls
                .into_values()
                .flat_map(|topic| topic.into_values())
                .flat_map(|partition| partition.into_values())
                .flat_map(|batch| batch.records)
                .for_each(|(_, tx)| {
                    let _ = tx.send(Err(ProduceError::ProducerClosed));
                });
        });

        debug!("Producer loop is closing");
    }

    /// Schedules record to be sent
    async fn add_message(&mut self, (record, tx): ProduceRequestMessage) {
        let partitions = match self
            .controller
            .get_metadata([record.topic.clone()].into(), ForceRefresh::No)
            .await
            .map(|x| x.into_values().next())
        {
            Ok(Some(MetadataResponseTopic {
                error_code: None,
                partitions,
                ..
            })) => partitions,
            Ok(Some(MetadataResponseTopic {
                error_code: Some(error_code),
                ..
            })) => {
                let error = match error_code {
                    ApiError::RequestTimedOut => {
                        ProduceError::ApiCallError(ApiCallError::TimeoutReached)
                    }
                    ApiError::UnknownTopicOrPartition => ProduceError::TopicNotFound(record.topic),
                    error_code => ProduceError::ApiCallError(ApiCallError::UnexpectedErrorCode(
                        MetadataRequest::get_api_key(),
                        error_code,
                        "topics.error_code",
                    )),
                };
                let _ = tx.send(Err(error));
                return;
            }
            Ok(None) => {
                let _ = tx.send(Err(ProduceError::TopicNotFound(record.topic)));
                return;
            }
            Err(err) => {
                let _ = tx.send(Err(ProduceError::MetadataFetchFailed(err)));
                return;
            }
        };

        let partition = record.partition.unwrap_or_else(|| {
            self.producer_options
                .partitioner
                .calculate_partition_index(&record.key, partitions.len())
        });

        let broker_id = match partitions.iter().find(|x| x.partition_index == partition) {
            Some(&MetadataResponsePartition {
                error_code: None,
                leader_id,
                ..
            }) => leader_id,
            Some(&MetadataResponsePartition {
                error_code: Some(error_code),
                ..
            }) => {
                let error = match error_code {
                    ApiError::UnknownTopicOrPartition => ProduceError::TopicNotFound(record.topic),
                    error_code => ProduceError::ApiCallError(ApiCallError::UnexpectedErrorCode(
                        MetadataRequest::get_api_key(),
                        error_code,
                        "topics.partitions.error_code",
                    )),
                };
                let _ = tx.send(Err(error));
                return;
            }
            None => {
                let _ = tx.send(Err(ProduceError::PartitionNotFound(
                    record.topic,
                    partition,
                )));
                return;
            }
        };

        let timestamp = record.timestamp.unwrap_or(SystemTime::now());

        let produce_record = Record {
            timestamp_delta: VarLong(
                timestamp
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64,
            ),
            offset_delta: Default::default(),
            key: VarIntBytes(record.key),
            value: VarIntBytes(record.value),
            headers: VarIntVec(
                record
                    .headers
                    .into_iter()
                    .map(|(key, value)| Header {
                        key: VarIntString(key),
                        value: VarIntBytes(value),
                    })
                    .collect(),
            ),
        };

        let record_append = RecordAppend {
            partition,
            offset: 0,
            timestamp,
        };

        let broker_state = self
            .brokers
            .entry(broker_id)
            .or_insert_with(|| BrokerState::new(self.producer_options.clone()));
        broker_state.add_record(record.topic, produce_record, record_append, tx);
    }

    /// Returns id of a broker, that is able to receive new records.
    ///
    /// If there are no messages to be sent future will never resolve.
    fn ready_to_send_data(&mut self) -> impl Future<Output = i32> {
        let (broker_id, delay) = self
            .brokers
            .iter()
            .find_map(|(&id, state)| {
                state
                    .records_waiting
                    .iter()
                    .find(|(_, partition_batches)| {
                        partition_batches
                            .iter()
                            .any(|batch| matches!(batch.state, BatchBuilderState::Full))
                    })
                    .map(|_| (id, None))
            })
            .or_else(|| {
                self.brokers
                    .iter()
                    .filter_map(|(&id, state)| state.linger.map(|delay| (id, delay)))
                    .min_by(|(_, x), (_, y)| x.cmp(y))
                    .map(|(broker, delay)| (broker, Some(delay)))
            })
            .unzip();

        async move {
            if let Some(broker_id) = broker_id {
                if let Some(delay) = delay.flatten() {
                    tokio::time::sleep_until(delay).await;
                }
                return broker_id;
            }
            future::pending().await
        }
    }

    /// Sends all accumulated records to the broker.
    ///
    /// If `broker_id=-1` pending promise will be returned for type system compatibility.
    fn send_produce_request(
        &mut self,
        broker_id: i32,
    ) -> Pin<Box<impl Future<Output = ProduceApiCallResult> + use<P>>> {
        if broker_id == -1 {
            return Box::pin(Either::Left(future::pending()));
        }

        self.produce_calls_counter = self.produce_calls_counter.overflowing_add(1).0;
        let msg_id = self.produce_calls_counter;

        let broker = self.brokers.get_mut(&broker_id).unwrap();
        broker.linger = None;

        let produce_call = broker.active_calls.entry(msg_id).or_default();

        let mut api_call = ProduceRequest {
            acks: self.producer_options.acks.into(),
            timeout_ms: self
                .producer_options
                .timeout
                .map(|d| d.as_millis() as i32)
                .unwrap_or(0),
            ..Default::default()
        };

        while let Some(((topic, partition), batch)) = broker.records_waiting.pop_first() {
            let (records, record_appends) = batch
                .into_iter()
                .flat_map(|x| x.records)
                .map(|(record, append, tx)| (record, (append, tx)))
                .unzip();

            produce_call.entry(topic.clone()).or_default().insert(
                partition,
                Batch {
                    records: record_appends,
                },
            );

            let mut record_batch = RecordBatch {
                records,
                ..Default::default()
            };

            record_batch.encode(&mut self.serialization_buffer);
            let records = Some(self.serialization_buffer.to_vec());
            self.serialization_buffer.clear();

            if api_call.topics.last().map(|topic| &topic.name) != Some(&topic) {
                api_call.topics.push(TopicProduceData {
                    name: topic.clone(),
                    partitions: vec![],
                });
            }

            api_call
                .topics
                .last_mut()
                .unwrap()
                .partitions
                .push(PartitionProduceData {
                    partition_index: partition,
                    records,
                });
        }

        let controller = self.controller.clone();
        let no_ack = matches!(self.producer_options.acks, Acks::NoAck);

        Box::pin(Either::Right(async move {
            ProduceApiCallResult {
                broker_id,
                msg_id,
                response: {
                    if no_ack {
                        controller
                            .make_api_call_without_response(broker_id, api_call, None)
                            .await
                            .map(|_| None)
                    } else {
                        controller
                            .make_api_call(broker_id, api_call, None)
                            .await
                            .map(Some)
                    }
                },
            }
        }))
    }

    async fn on_produce_response(
        &mut self,
        ProduceApiCallResult {
            broker_id,
            msg_id,
            response,
        }: ProduceApiCallResult,
    ) {
        let mut records_sent = self
            .brokers
            .get_mut(&broker_id)
            .unwrap()
            .active_calls
            .remove(&msg_id)
            .unwrap();

        match response {
            Ok(Some(ProduceResponse { responses, .. })) => {
                let mappings = responses
                    .into_iter()
                    .map(|topic| (records_sent.remove(&topic.name).unwrap(), topic.partitions))
                    .flat_map(|(mut batches, partition_responses)| {
                        partition_responses
                            .into_iter()
                            .map(|partition_response| {
                                (
                                    batches.remove(&partition_response.partition_index).unwrap(),
                                    partition_response,
                                )
                            })
                            .collect::<Vec<_>>()
                    });

                for (partition_records, partition_resp) in mappings {
                    match partition_resp.error_code {
                        None => {
                            for (i, (mut append, tx)) in
                                partition_records.records.into_iter().enumerate()
                            {
                                append.offset = partition_resp.base_offset + i as i64;
                                if partition_resp.log_append_time_ms != -1 {
                                    append.timestamp =
                                        SystemTime::UNIX_EPOCH.add(Duration::from_millis(
                                            partition_resp.log_append_time_ms as u64,
                                        ));
                                }
                                let _ = tx.send(Ok(append));
                            }
                        }
                        Some(error_code) => {
                            partition_records.records.into_iter().for_each(|(_, tx)| {
                                let _ = tx.send(Err(ProduceError::ApiCallError(
                                    ApiCallError::UnexpectedErrorCode(
                                        ProduceResponse::get_api_key(),
                                        error_code,
                                        "responses.partitions.error_code",
                                    ),
                                )));
                            });
                        }
                    }
                }
            }
            Ok(None) => {
                records_sent
                    .into_values()
                    .flat_map(|topic| topic.into_values())
                    .flat_map(|batch| batch.records)
                    .for_each(|(mut append, tx)| {
                        append.offset = -1;
                        let _ = tx.send(Ok(append));
                    });
            }
            Err(err) => {
                records_sent
                    .into_values()
                    .flat_map(|topic| topic.into_values())
                    .flat_map(|batch| batch.records)
                    .for_each(|(_, tx)| {
                        let _ = tx.send(Err(ProduceError::ApiCallError(err.clone())));
                    });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod broker_state {
        use super::*;
        mod add_record {
            use super::*;

            #[test_log::test]
            fn sets_linger_only_on_first_record() {
                let options = Arc::new(KafkaProducerOptions::new());
                let mut state = BrokerState::new(options);

                assert!(state.linger.is_none());

                let produce_record = Record::default();
                let record_append = RecordAppend {
                    partition: 0,
                    offset: 0,
                    timestamp: SystemTime::now(),
                };

                let (tx, _) = oneshot::channel();
                state.add_record(
                    "topic".to_owned(),
                    produce_record.clone(),
                    record_append.clone(),
                    tx,
                );

                assert!(state.linger.is_some());
                let prev_value = state.linger.as_ref().unwrap().clone();

                let (tx, _) = oneshot::channel();
                state.add_record(
                    "topic".to_owned(),
                    produce_record.clone(),
                    record_append.clone(),
                    tx,
                );
                assert!(state.linger.is_some());
                assert_eq!(state.linger.unwrap(), prev_value);
            }

            #[test_log::test]
            fn creates_new_batch_once_batch_size_limit_is_met() {
                let options = Arc::new(KafkaProducerOptions {
                    batch_size_bytes: 1_000,
                    ..KafkaProducerOptions::new()
                });
                let mut state = BrokerState::new(options);
                let topic_name = "topic".to_owned();

                let record_small = Record {
                    value: VarIntBytes([0_u8; 494].into()),
                    ..Default::default()
                };
                assert_eq!(record_small.estimate_size(), 500);

                let record_large = Record {
                    value: VarIntBytes([0_u8; 2_000].into()),
                    ..Default::default()
                };

                let record_append = RecordAppend {
                    partition: 0,
                    offset: 0,
                    timestamp: SystemTime::now(),
                };

                state.add_record(
                    topic_name.clone(),
                    record_small.clone(),
                    record_append.clone(),
                    oneshot::channel().0,
                );
                let mut batches = state.records_waiting.iter().next().unwrap().1.iter();
                assert_eq!(
                    batches.next().unwrap().state,
                    BatchBuilderState::UnderConstruction { bytes_left: 500 }
                );
                assert!(batches.next().is_none());

                state.add_record(
                    topic_name.clone(),
                    record_small.clone(),
                    record_append.clone(),
                    oneshot::channel().0,
                );
                let mut batches = state.records_waiting.iter().next().unwrap().1.iter();
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert!(batches.next().is_none());

                state.add_record(
                    topic_name.clone(),
                    record_small.clone(),
                    record_append.clone(),
                    oneshot::channel().0,
                );
                let mut batches = state.records_waiting.iter().next().unwrap().1.iter();
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert_eq!(
                    batches.next().unwrap().state,
                    BatchBuilderState::UnderConstruction { bytes_left: 500 }
                );
                assert!(batches.next().is_none());

                state.add_record(
                    topic_name.clone(),
                    record_large.clone(),
                    record_append.clone(),
                    oneshot::channel().0,
                );
                let mut batches = state.records_waiting.iter().next().unwrap().1.iter();
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert!(batches.next().is_none());

                state.add_record(
                    topic_name.clone(),
                    record_large.clone(),
                    record_append.clone(),
                    oneshot::channel().0,
                );
                let mut batches = state.records_waiting.iter().next().unwrap().1.iter();
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert_eq!(batches.next().unwrap().state, BatchBuilderState::Full);
                assert!(batches.next().is_none());
            }
        }
    }
}
