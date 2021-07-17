use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::cluster::{metadata::Metadata, ClusterInner};
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
use log::{trace, warn};
use tokio::sync::{mpsc::UnboundedReceiver, oneshot};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use super::{error::MessageSendError, options::ProducerOptions, record::ProducerRecord};

pub type RecordStatusReporter = oneshot::Sender<Result<(), (MessageSendError, ProducerRecord)>>;

#[derive(Debug)]
pub(super) enum ProducerLoopSignal {
    /// Queue message for sending
    SendMessage(ProducerRecord, RecordStatusReporter),
    /// Send messages waiting in a queue
    SendBatch,
    /// Disconnect from kafka brokers, clean up
    Shutdown,
}

pub(super) struct ProducerLoop {
    pub cluster: Arc<ClusterInner>,
    pub options: Arc<ProducerOptions>,
}
impl ProducerLoop {
    pub async fn producer_loop(self, loop_signal_receiver: UnboundedReceiver<ProducerLoopSignal>) {
        let mut message_queue = Vec::new();

        let mut stream = UnboundedReceiverStream::new(loop_signal_receiver);

        while let Some(signal) = stream.next().await {
            match signal {
                ProducerLoopSignal::SendMessage(message, ack_result) => {
                    message_queue.push((message, ack_result));
                }
                ProducerLoopSignal::SendBatch => {
                    if message_queue.is_empty() {
                        trace!("No messages on producer queue to send");
                        continue;
                    };

                    self.send_batch(&mut message_queue).await;
                }
                ProducerLoopSignal::Shutdown => {
                    break;
                }
            }
        }
        for message in message_queue {
            if message
                .1
                .send(Err((MessageSendError::ProducerClosed, message.0)))
                .is_err()
            {
                warn!("Message not received without client knowledge");
            }
        }
        trace!("Producer loop close")
    }
    async fn send_batch(&self, message_queue: &mut Vec<(ProducerRecord, RecordStatusReporter)>) {
        self.fetch_topic_metadata(message_queue).await;
        let broker_map = self.group_records_by_brokers(message_queue).await;

        for broker_records in broker_map {
            self.send_records_to_broker(broker_records).await;
        }
    }

    async fn send_records_to_broker(
        &self,
        broker_records: (i32, Vec<(ProducerRecord, RecordStatusReporter)>),
    ) {
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
                    .map(|x| (x.0.partition(), x))
                    .fold(HashMap::new(), group_by_first_element)
                    .into_iter()
                    .map(|x| ProduceRequestTopicDataData0 {
                        partition: x.0,
                        record_set: RecordBatchWithSize {
                            batches: vec![convert_partition_records_to_record_batch(
                                x.1.into_iter().map(|x| x.0),
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

        // TODO: Unwrap
        // TODO: Check for errors on specific messages
        // TODO: Send acks to producers
        self.cluster
            .run_api_call_on_specific_broker(broker_records.0, request, None)
            .await
            .unwrap();
    }

    async fn group_records_by_brokers(
        &self,
        message_queue: &mut Vec<(ProducerRecord, RecordStatusReporter)>,
    ) -> HashMap<i32, Vec<(ProducerRecord, RecordStatusReporter)>> {
        let metadata = self.cluster.metadata.read().await;
        message_queue
            .drain(..)
            .map(|x| (get_broker_id_for_partition_id(&x.0, &*metadata), x))
            .fold(HashMap::new(), group_by_first_element)
    }

    async fn fetch_topic_metadata(
        &self,
        message_queue: &mut Vec<(ProducerRecord, RecordStatusReporter)>,
    ) {
        let topics_lacking_metadata = {
            let topics = message_queue
                .iter()
                .map(|record| record.0.topic.as_str())
                .collect::<HashSet<_>>();
            let metadata = self.cluster.metadata.read().await;
            topics
                .into_iter()
                .filter(|key| !metadata.topics.contains_key(*key))
                .map(str::to_owned)
                .collect::<Vec<_>>()
        };
        if !topics_lacking_metadata.is_empty() {
            // TODO: Handle errors
            self.cluster
                .fetch_topic_metadata(topics_lacking_metadata)
                .await
                .unwrap();
        }
    }
}

fn convert_partition_records_to_record_batch<I>(records: I) -> RecordBatch
where
    I: IntoIterator<Item = ProducerRecord>,
{
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
                timestamp: x.timestamp.unwrap_or_default(),
                offset: 0,
                key: ZigZagVec { value: x.key },
                value: ZigZagVec { value: x.payload },
                headers: ZigZagVec {
                    value: x
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

fn group_by_first_element<T>(
    mut map: HashMap<i32, Vec<T>>,
    item: (i32, T),
) -> HashMap<i32, Vec<T>> {
    if let Some(x) = map.get_mut(&item.0) {
        x.push(item.1);
    } else {
        map.insert(item.0, vec![item.1]);
    };
    map
}

fn get_broker_id_for_partition_id(record: &ProducerRecord, metadata: &Metadata) -> i32 {
    // TODO: Remove unwraps
    let topic_metadata = metadata.topics.get(&record.topic).unwrap();
    topic_metadata
        .partitions
        .get(&record.partition())
        .unwrap()
        .leader_id
}
