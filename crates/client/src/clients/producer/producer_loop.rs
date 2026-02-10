use crate::clients::producer::client::{KafkaProducerOptions, RecordAppend};
use crate::clients::producer::error::ProduceError;
use crate::clients::producer::future_record::FutureRecord;
use crate::cluster::controller::{ClusterController, ForceRefresh};
use crate::cluster::error::ApiCallError;
use bytes::BytesMut;
use futures::future::select_all;
use kafka_connector_protocol::metadata_request::MetadataRequest;
use kafka_connector_protocol::metadata_response::MetadataResponseTopic;
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
use kafka_connector_protocol::{ApiError, ApiRequest};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, instrument};

pub(super) type ProduceRequestMessage = (FutureRecord, RecordAppendTx);
pub(super) type RecordAppendTx = oneshot::Sender<Result<RecordAppend, ProduceError>>;

type ProduceApiCallResult = (i32, Result<ProduceResponse, ApiCallError>);
type RecordBatchesToSend =
    BTreeMap<RecordBatchID, (RecordBatch, Vec<(RecordAppend, RecordAppendTx)>)>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct RecordBatchID {
    broker_id: i32,
    topic: String,
    partition: i32,
}
pub struct ProducerLoop {
    controller: Arc<ClusterController>,
    producer_options: KafkaProducerOptions,
    serialization_buffer: BytesMut,
    records_waiting: VecDeque<ProduceRequestMessage>,
    records_in_flight: BTreeMap<RecordBatchID, Vec<(RecordAppend, RecordAppendTx)>>,
}

impl ProducerLoop {
    #[instrument(level = "debug", skip_all)]
    pub async fn start(
        controller: Arc<ClusterController>,
        producer_options: KafkaProducerOptions,
        receiver: mpsc::UnboundedReceiver<ProduceRequestMessage>,
    ) {
        ProducerLoop {
            controller,
            producer_options,
            serialization_buffer: BytesMut::with_capacity(10_000_000), // TODO: size from options, at least 2 times max record_batch, default same as for rdkafka
            records_waiting: VecDeque::new(),
            records_in_flight: Default::default(),
        }
        .run(receiver)
        .await;
    }

    #[instrument(level = "debug", skip_all)]
    async fn run(&mut self, mut receiver: mpsc::UnboundedReceiver<ProduceRequestMessage>) {
        debug!("Producer loop started.");
        // TODO: Add batching based on time

        let mut send_message_task = Box::pin(Self::conditional_task(None));

        loop {
            tokio::select! {
                signal = receiver.recv() => {
                    match signal {
                        Some(request) => {
                            self.records_waiting.push_back(request);
                            if self.records_in_flight.is_empty() {
                                if let Some(futures) = self.send_messages().await {
                                    send_message_task = Box::pin(Self::conditional_task(Some(select_all(futures))));
                                }
                            }
                        }
                        None => { break;}
                    }
                },
                Some((response, _, api_calls_in_flight)) = &mut send_message_task, if !self.records_in_flight.is_empty() => {
                    self.handle_responses(response).await;
                    if api_calls_in_flight.is_empty() {
                         if let Some(futures) = self.send_messages().await {
                            send_message_task = Box::pin(Self::conditional_task(Some(select_all(futures))));
                        }
                    } else {
                        send_message_task = Box::pin(Self::conditional_task(Some(select_all(api_calls_in_flight))));
                    }

                }
            }
        }
        // TODO: Cleanup waiting/in-flight requests

        debug!("Producer loop is closing");

        // TODO: Make sure it closes when producer client is dropped - test?
    }

    async fn handle_responses(
        &mut self,
        (broker_id, response): (i32, Result<ProduceResponse, ApiCallError>),
    ) {
        // TODO: error handling
        let response = response.unwrap();
        for resp in response.responses {
            for part in resp.partitions {
                assert!(part.error_code.is_none());

                let partition_signals = self
                    .records_in_flight
                    .remove(&RecordBatchID {
                        broker_id,
                        topic: resp.name.clone(),
                        partition: part.partition_index,
                    })
                    .unwrap();
                for (i, (mut append, sig)) in partition_signals.into_iter().enumerate() {
                    append.offset = part.base_offset + i as i64;
                    if part.log_append_time_ms != -1 {
                        append.timestamp = SystemTime::UNIX_EPOCH
                            .add(Duration::from_millis(part.log_append_time_ms as u64));
                    }
                    let _ = sig.send(Ok(append));
                }
            }
        }
    }

    async fn conditional_task<F: Future<Output = OUT>, OUT>(t: Option<F>) -> Option<OUT> {
        match t {
            None => None,
            Some(t) => Some(t.await),
        }
    }
    async fn send_messages(
        &mut self,
    ) -> Option<
        Vec<Pin<Box<impl Future<Output = (i32, Result<ProduceResponse, ApiCallError>)> + use<>>>>,
    > {
        // TODO: max record batch size - may require changing single record batch to vec (if max msg size > max record batch size)
        // TODO: KIP-192 retry, multiple calls in transit (to same broker) - most retries should be handled by producer client, some (idempotency related) by loop itself

        if self.records_waiting.is_empty() {
            return None;
        }

        let topics = self
            .records_waiting
            .iter()
            .map(|(record, _)| record.topic.clone())
            .collect::<HashSet<String>>();

        // Blocks producer loop if metadata requests is send to a broker - will prevent multiple requests for the same topics running in parallel
        let metadata = self
            .controller
            .get_topic_metadata(topics, ForceRefresh::No)
            .await
            .map_err(|err| {
                let err = Arc::new(err);
                while let Some(req) = self.records_waiting.pop_front() {
                    let _ = req
                        .1
                        .send(Err(ProduceError::MetadataFetchFailed(err.clone())));
                }
            })
            .ok()?;

        let mut batches_to_send = BTreeMap::new();
        let default_timestamp = SystemTime::now();

        while let Some((record, tx)) = self.records_waiting.pop_front() {
            if let Err((tx, err)) = process_record(
                record,
                &metadata,
                &mut batches_to_send,
                default_timestamp,
                tx,
            ) {
                let _ = tx.send(Err(err));
                continue;
            }
        }

        if batches_to_send.is_empty() {
            return None;
        }

        Some(self.send_produce_api_calls(&mut batches_to_send))
    }

    fn send_produce_api_calls(
        &mut self,
        batches_to_send: &mut RecordBatchesToSend,
    ) -> Vec<Pin<Box<impl Future<Output = ProduceApiCallResult> + use<>>>> {
        let mut requests: Vec<(i32, ProduceRequest)> = vec![];

        let mut prev_broker_id = None;
        let mut prev_topic = None;

        while let Some((
            RecordBatchID {
                broker_id,
                topic,
                partition,
            },
            (mut batch, signal_triggers),
        )) = batches_to_send.pop_first()
        {
            self.records_in_flight.insert(
                RecordBatchID {
                    broker_id,
                    topic: topic.clone(),
                    partition,
                },
                signal_triggers,
            );

            batch.encode(&mut self.serialization_buffer);
            let records = self.serialization_buffer.to_vec();
            self.serialization_buffer.clear();

            if prev_broker_id == Some(broker_id) {
                let topics = &mut requests.last_mut().unwrap().1.topics;
                if prev_topic.as_ref().unwrap() == topic.as_str() {
                    topics
                        .last_mut()
                        .unwrap()
                        .partitions
                        .push(PartitionProduceData {
                            partition_index: partition,
                            records: Some(records),
                        });
                } else {
                    topics.push(TopicProduceData {
                        name: topic.clone(),
                        partitions: vec![PartitionProduceData {
                            partition_index: partition,
                            records: Some(records),
                        }],
                    });
                }
            } else {
                requests.push((
                    broker_id,
                    ProduceRequest {
                        acks: 1, // TODO: Acks
                        topics: vec![TopicProduceData {
                            name: topic.clone(),
                            partitions: vec![PartitionProduceData {
                                partition_index: partition,
                                records: Some(records),
                            }],
                        }],
                        timeout_ms: 0, // TODO:
                        ..Default::default()
                    },
                ));
            }
            prev_broker_id = Some(broker_id);
            prev_topic = Some(topic);
        }

        requests
            .into_iter()
            .map(|(broker_id, request)| {
                let controller = self.controller.clone();
                Box::pin(async move {
                    (
                        broker_id,
                        controller.make_api_call(broker_id, request, None).await,
                    )
                })
            })
            .collect::<Vec<_>>()
    }
}

fn process_record(
    record: FutureRecord,
    metadata: &HashMap<String, MetadataResponseTopic>,
    batches_to_send: &mut RecordBatchesToSend,
    default_timestamp: SystemTime,
    tx: RecordAppendTx,
) -> Result<(), (RecordAppendTx, ProduceError)> {
    let topic_metadata = metadata.get(&record.topic).unwrap();
    if let Some(error_code) = topic_metadata.error_code {
        let error = match error_code {
            ApiError::RequestTimedOut => ProduceError::ApiCallError(ApiCallError::TimeoutReached),
            ApiError::UnknownTopicOrPartition => ProduceError::TopicNotFound(record.topic.clone()),
            error_code => ProduceError::ApiCallError(ApiCallError::UnsupportedErrorCode(
                MetadataRequest::get_api_key(),
                error_code,
                "topics.error_code",
            )),
        };
        return Err((tx, error));
    }

    let partition_idx = get_partition_no(&record, topic_metadata);
    let Some(partition_metadata) = topic_metadata
        .partitions
        .iter()
        .find(|x| x.partition_index == partition_idx)
    else {
        return Err((
            tx,
            ProduceError::PartitionNotFound(record.topic.clone(), partition_idx),
        ));
    };

    if let Some(error_code) = partition_metadata.error_code {
        let error = match error_code {
            ApiError::UnknownTopicOrPartition => ProduceError::TopicNotFound(record.topic.clone()),
            error_code => ProduceError::ApiCallError(ApiCallError::UnsupportedErrorCode(
                MetadataRequest::get_api_key(),
                error_code,
                "topics.partitions.error_code",
            )),
        };
        return Err((tx, error));
    }

    let broker_id = partition_metadata.leader_id;
    let timestamp = record.timestamp.unwrap_or(default_timestamp);
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

    let entry = batches_to_send
        .entry(RecordBatchID {
            broker_id,
            topic: record.topic,
            partition: partition_idx,
        })
        .or_default();
    entry.0.records.push(produce_record);
    entry.1.push((
        RecordAppend {
            partition: partition_idx,
            offset: 0,
            timestamp,
        },
        tx,
    ));

    Ok(())
}

fn get_partition_no(_record: &FutureRecord, _metadata: &MetadataResponseTopic) -> i32 {
    // TODO: Partitioner
    0
}
