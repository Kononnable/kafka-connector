use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::cluster::{cluster_loop::BrokerState, metadata::Metadata, ClusterInner};
use kafka_connector_protocol::{
    api::produce::{ProduceRequest, ProduceRequestTopicData0, ProduceRequestTopicDataData0},
    custom_types::{
        nullable_string::NullableString,
        record_batch::{
            batch::RecordBatch, header::Header, record::Record,
            record_batch_with_size::RecordBatchWithSize,
        },
        zig_zag_string::ZigZagString,
        zig_zag_vec::ZigZagVec,
    },
};
use log::trace;
use tokio::sync::{mpsc::UnboundedReceiver, oneshot};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use super::record::ProducerRecord;

#[derive(Debug)]
pub(super) enum ProducerLoopSignal {
    /// Queue message for sending
    SendMessage(ProducerRecord, oneshot::Sender<()>),
    /// Send messages waiting in a queue
    SendBatch,
    /// Disconnect from kafka brokers, clean up
    Shutdown,
}

pub(super) async fn producer_loop(
    loop_signal_receiver: UnboundedReceiver<ProducerLoopSignal>,
    cluster: Arc<ClusterInner>,
) {
    let mut message_queue: Vec<(ProducerRecord, oneshot::Sender<()>)> = Vec::new();

    let send_batch_timeout = futures::stream::repeat_with(|| ProducerLoopSignal::SendBatch)
        .throttle(Duration::from_secs(2)); // TODO: configurable,  change value

    let mut stream =
        Box::pin(send_batch_timeout.merge(UnboundedReceiverStream::new(loop_signal_receiver)));

    while let Some(signal) = stream.next().await {
        match signal {
            ProducerLoopSignal::SendMessage(message, ack_result) => {
                message_queue.push((message, ack_result));
            }
            ProducerLoopSignal::SendBatch => {
                // TODO: Unwraps
                if message_queue.is_empty() {
                    trace!("No messages on producer queue to send");
                    continue;
                };
                let broker_map = {
                    let metadata = cluster.metadata.read().await;
                    let x = message_queue
                        .into_iter()
                        .map(|x| (generate_partition_number(x.0), x.1))
                        .map(|x| (get_broker_id_for_partition_id(&x.0, &*metadata), x))
                        .fold(HashMap::new(), group_by_first_element);
                    message_queue = Vec::new();
                    x
                };
                for broker_records in broker_map {
                    let mut brokers = cluster.brokers.write().await;
                    let broker = brokers.get_mut(&broker_records.0).unwrap();
                    let topic_grouped = broker_records.1.into_iter().fold(
                        HashMap::new(),
                        |mut a: HashMap<String, Vec<_>>, b| {
                            if let Some(x) = a.get_mut(&b.0.topic) {
                                x.push(b);
                            } else {
                                a.insert(b.0.topic.clone(), vec![b]);
                            };
                            a
                        },
                    );
                    let topic_data = topic_grouped
                        .into_iter()
                        .map(|data| ProduceRequestTopicData0 {
                            topic: data.0,
                            data: data
                                .1
                                .into_iter()
                                .map(|x| (x.0.partition.unwrap(), x))
                                .fold(HashMap::new(), group_by_first_element)
                                .into_iter()
                                .map(|x| ProduceRequestTopicDataData0 {
                                    partition: x.0,
                                    record_set: RecordBatchWithSize {
                                        batches: vec![convert_partition_records_to_record_batch(
                                            x.1,
                                        )],
                                    },
                                })
                                .collect::<Vec<ProduceRequestTopicDataData0>>(),
                        })
                        .collect::<Vec<_>>();
                    let request = ProduceRequest {
                        transactional_id: NullableString::None,
                        acks: 1, // TODO: Acks config
                        timeout: 30_000,
                        topic_data,
                    };
                    match broker {
                        BrokerState::Alive { broker, addr } => {
                            broker.run_api_call_with_retry(request, None).await.unwrap();
                        }
                        BrokerState::Initializing { addr } => {
                            todo!()
                        }
                    }
                }
                // TODO: Acks
            }
            ProducerLoopSignal::Shutdown => {
                break;
            }
        }
    }
    // TODO: Return error to all messages in message_queue, messages already sent, but without ack
    trace!("Producer loop close")
}

fn convert_partition_records_to_record_batch(
    records: Vec<(ProducerRecord, oneshot::Sender<()>)>,
) -> RecordBatch {
    // TODO: Proper fields values
    RecordBatch {
        attributes: 0,
        base_sequence: -1,
        partition_leader_epoch: 0,
        producer_epoch: -1,
        producer_id: -1,
        records: records
            .into_iter()
            .map(|x| Record {
                attributes: 0,
                timestamp: x.0.timestamp.unwrap_or_default(),
                offset: 0,
                key: ZigZagVec { value: x.0.key },
                value: ZigZagVec { value: x.0.payload },
                headers: ZigZagVec {
                    value: x
                        .0
                        .headers
                        .unwrap_or_default()
                        .into_iter()
                        .map(|z| Header {
                            key: ZigZagString { value: z.0 },
                            value: ZigZagVec { value: z.1 },
                        })
                        .collect(),
                },
            })
            .collect(),
    }
}

fn group_by_first_element<T>(mut a: HashMap<i32, Vec<T>>, b: (i32, T)) -> HashMap<i32, Vec<T>> {
    if let Some(x) = a.get_mut(&b.0) {
        x.push(b.1);
    } else {
        a.insert(b.0, vec![b.1]);
    };
    a
}

fn get_broker_id_for_partition_id(record: &ProducerRecord, metadata: &Metadata) -> i32 {
    // TODO: Remove unwraps
    let topic_metadata = metadata.topics.get(&record.topic).unwrap();
    trace!("{:?}", record);
    trace!("{:?}", topic_metadata);
    topic_metadata
        .partitions
        .get(&record.partition.unwrap())
        .unwrap()
        .leader_id
}
fn generate_partition_number(mut record: ProducerRecord) -> ProducerRecord {
    record.partition.get_or_insert_with(Default::default);
    record
    // TODO: Partitioner
}