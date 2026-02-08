use crate::clients::producer::client::{KafkaProducerOptions, ProduceRequestMessage, RecordAppend};
use crate::clients::producer::error::ProduceError;
use crate::clients::producer::future_record::FutureRecord;
use crate::cluster::controller::{ClusterController, ForceRefresh};
use crate::cluster::error::ApiCallError;
use bytes::BytesMut;
use kafka_connector_protocol::metadata_request::MetadataRequest;
use kafka_connector_protocol::metadata_response::MetadataResponseTopic;
use kafka_connector_protocol::produce_request::{
    PartitionProduceData, ProduceRequest, TopicProduceData,
};
use kafka_connector_protocol::records::base_types::{
    VarIntBytes, VarIntString, VarIntVec, VarLong,
};
use kafka_connector_protocol::records::header::Header;
use kafka_connector_protocol::records::record::Record;
use kafka_connector_protocol::records::record_batch::RecordBatch;
use kafka_connector_protocol::{ApiError, ApiRequest};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, instrument};

pub struct ProducerLoop {
    controller: Arc<ClusterController>,
    producer_options: KafkaProducerOptions,
    serialization_buffer: Mutex<BytesMut>,
    waiting_queue: Mutex<
        VecDeque<(
            FutureRecord,
            oneshot::Sender<Result<RecordAppend, ProduceError>>,
        )>,
    >,
    in_flight_requests: Option<()>,
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
            serialization_buffer: Mutex::new(BytesMut::with_capacity(10_000_000)), // TODO: size from options, at least 2 times max record_batch, default same as for rdkafka
            waiting_queue: Mutex::new(VecDeque::new()),
            in_flight_requests: None,
        }
        .run(receiver)
        .await;
    }

    #[instrument(level = "debug", skip(self))]
    async fn run(&self, mut receiver: mpsc::UnboundedReceiver<ProduceRequestMessage>) {
        debug!("Producer loop started.");
        // TODO: Add batching based on time
        // TODO: Add concurrency (loop with select! and queues) - nonblocking

        let mut send_message_task = None;
        loop {
            tokio::select! {
                signal = receiver.recv() => {
                    match signal {
                        Some(request) => {
                            {
                            let mut waiting_queue = self.waiting_queue.lock().expect("Poisoned lock");
                            for msg in request.records {
                                waiting_queue.push_back( msg);
                            }
                            drop(waiting_queue);
                                }
                            if self.in_flight_requests.is_none() {
                                 send_message_task = self.send_messages().await;
                                // TODO: run in a non-blocking way within a loop(select)
                                if let Some(x) = send_message_task.take(){
                                    x.await;
                                }
                            }
                        }
                        None => { break;}
                    }
                }
                // TODO: receive response

            }
        }
        // TODO: Cleanup waiting/in-flight requests

        debug!("Producer loop is closing");

        // TODO: Make sure it closes when producer client is dropped - test?
    }

    async fn conditional_task<F: Future<Output = Out>, Out>(f: Option<F>) -> Option<Out> {
        match f {
            Some(f) => Some(f.await),
            None => None,
        }
    }
    async fn send_messages(&self) -> Option<impl Future<Output = ()>> {
        // TODO: max record batch size - may require changing single record batch to vec (if max msg size > max record batch size)
        // TODO: retry with backoff
        // TODO: multiple records in transit
        // TODO: partition leader change (on retry)

        let topics = {
            let waiting_queue = self.waiting_queue.lock().expect("Poisoned lock");
            if waiting_queue.is_empty() {
                return None;
            }
            waiting_queue
                .iter()
                .map(|(record, _)| record.topic.clone())
                .collect::<HashSet<String>>()
        };

        // Blocks producer loop if metadata requests is send to a broker - will prevent multiple request for the same topics running in parallel
        let metadata = self
            .controller
            .get_topic_metadata(topics, ForceRefresh::No)
            .await // TODO: error handling
            .unwrap();

        let mut records_to_send = BTreeMap::<
            (i32, String, i32),
            (
                RecordBatch,
                Vec<(
                    RecordAppend,
                    oneshot::Sender<Result<RecordAppend, ProduceError>>,
                )>,
            ),
        >::new();

        let default_timestamp = SystemTime::now();

        // TODO: change to tokio mutex? reacquiring mutex while no one else could take it (same thread as main loop)
        {
            let mut waiting_queue = self.waiting_queue.lock().expect("Poisoned lock");
            while let Some((record, tx)) = waiting_queue.pop_front() {
                let x = process_record(
                    record,
                    &metadata,
                    &mut records_to_send,
                    default_timestamp,
                    tx,
                );
                if let Err(()) = x {
                    continue;
                }
            }
        }

        let mut requests = vec![];
        let mut signal_handlers = BTreeMap::<
            (i32, String, i32),
            Vec<(
                RecordAppend,
                oneshot::Sender<Result<RecordAppend, ProduceError>>,
            )>,
        >::new();

        let ((mut prev_broker_id, mut prev_topic, partition), (mut batch, signal_triggers)) =
            records_to_send.pop_first().unwrap();
        signal_handlers.insert(
            (prev_broker_id, prev_topic.clone(), partition),
            signal_triggers,
        );

        let mut serialization_buffer = self.serialization_buffer.lock().expect("Poisoned lock");
        batch.encode(&mut serialization_buffer);
        let records = serialization_buffer.to_vec();
        serialization_buffer.clear();
        requests.push((
            prev_broker_id,
            ProduceRequest {
                acks: 1, // TODO: Acks
                topics: vec![TopicProduceData {
                    name: prev_topic.to_string(),
                    partitions: vec![PartitionProduceData {
                        partition_index: partition,
                        records: Some(records),
                    }],
                }],
                ..Default::default()
            },
        ));

        while let Some(((broker_id, topic, partition), (mut batch, signal_triggers))) =
            records_to_send.pop_first()
        {
            signal_handlers.insert((broker_id, topic.clone(), partition), signal_triggers);

            batch.encode(&mut serialization_buffer);
            let records = serialization_buffer.to_vec();
            serialization_buffer.clear();
            if prev_broker_id == broker_id {
                if prev_topic == topic {
                    requests
                        .last_mut()
                        .unwrap()
                        .1
                        .topics
                        .last_mut()
                        .unwrap()
                        .partitions
                        .push(PartitionProduceData {
                            partition_index: partition,
                            records: Some(records),
                        });
                } else {
                    requests
                        .last_mut()
                        .unwrap()
                        .1
                        .topics
                        .push(TopicProduceData {
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
                        ..Default::default()
                    },
                ));
            }
            prev_broker_id = broker_id;
            prev_topic = topic;
        }

        let controller = self.controller.clone();
        Some(async move {
            for (broker_id, request) in requests {
                let resp = controller
                    .make_api_call(broker_id, request, None)
                    .await
                    .unwrap(); // TODO: in background, error handling

                for resp in resp.responses {
                    for part in resp.partitions {
                        assert!(part.error_code.is_none());

                        let partition_signals = signal_handlers
                            .remove(&(broker_id, resp.name.clone(), part.partition_index))
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
        })
    }
}

fn process_record(
    record: FutureRecord,
    metadata: &HashMap<String, MetadataResponseTopic>,
    records_to_send: &mut BTreeMap<
        (i32, String, i32),
        (
            RecordBatch,
            Vec<(
                RecordAppend,
                oneshot::Sender<Result<RecordAppend, ProduceError>>,
            )>,
        ),
    >,
    default_timestamp: SystemTime,
    tx: oneshot::Sender<Result<RecordAppend, ProduceError>>,
) -> Result<(), ()> {
    let topic_metadata = metadata.get(&record.topic).unwrap();
    if let Some(error_code) = topic_metadata.error_code {
        let err = match error_code {
            ApiError::RequestTimedOut => ProduceError::ApiCallError(ApiCallError::TimeoutReached),
            ApiError::UnknownTopicOrPartition => ProduceError::TopicNotFound(record.topic.clone()),
            error_code => ProduceError::ApiCallError(ApiCallError::UnsupportedErrorCode(
                MetadataRequest::get_api_key(),
                error_code,
                "topics.error_code",
            )),
        };
        let _ = tx.send(Err(err));
        return Err(());
    }

    let partition_idx = get_partition_no(&record, topic_metadata);
    let partition_metadata = topic_metadata
        .partitions
        .iter()
        .find(|x| x.partition_index == partition_idx)
        .ok_or_else(|| ProduceError::PartitionNotFound(record.topic.clone(), partition_idx));

    let partition_metadata = if let Err(err) = partition_metadata {
        let _ = tx.send(Err(err));
        return Err(());
    } else {
        partition_metadata.unwrap()
    };

    if let Some(error_code) = partition_metadata.error_code {
        let err = match error_code {
            ApiError::UnknownTopicOrPartition => ProduceError::TopicNotFound(record.topic.clone()),
            error_code => ProduceError::ApiCallError(ApiCallError::UnsupportedErrorCode(
                MetadataRequest::get_api_key(),
                error_code,
                "topics.partitions.error_code",
            )),
        };
        let _ = tx.send(Err(err));
        return Err(());
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

    let record_entry = records_to_send
        .entry((broker_id, record.topic, partition_idx))
        .or_default();
    record_entry.0.records.push(produce_record);
    record_entry.1.push((
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
