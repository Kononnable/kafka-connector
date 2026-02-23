use crate::clients::consumer::client::KafkaConsumerOptions;
use crate::clients::consumer::error::ConsumeError;
use crate::clients::consumer::error::ConsumeError::MetadataFetchFailed;
use crate::clients::consumer::record::Record;
use crate::cluster::controller::{ClusterController, ForceRefresh};
use crate::cluster::error::ApiCallError;
use bytes::BytesMut;
use futures::future::{join_all, select_all};
use kafka_connector_protocol::fetch_request::{FetchPartition, FetchRequest, FetchableTopic};
use kafka_connector_protocol::fetch_response::FetchResponse;
use kafka_connector_protocol::list_offset_request::{
    ListOffsetPartition, ListOffsetRequest, ListOffsetTopic,
};
use kafka_connector_protocol::records::record_batch::RecordBatch;
use std::collections::HashMap;
use std::ops::Add;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, instrument};

struct PartitionMapping {
    broker_id: i32,
    topic: String,
    partition_id: i32,
}
pub struct ConsumerLoop {
    controller: Arc<ClusterController>,
    consumer_options: KafkaConsumerOptions,
    record_sender: mpsc::Sender<Record>,
    command_receiver: mpsc::Receiver<()>,
    mappings: HashMap<i32, PartitionMapping2>,
}

type PartitionMapping2 = HashMap<String, HashMap<i32, i64>>;

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
            command_receiver,
            mappings: Default::default(),
        }
        .run()
        .await;
    }

    #[instrument(level = "debug", skip(self))]
    async fn run(&mut self) {
        debug!("Consumer loop started.");
        // TODO: Proper loop, consumer groups etc.
        // TODO: Multiple topics support
        // TODO: asserts

        // TODO: Error handling

        // TODO: Support fetching data from isr replicas
        self.mappings = self.prepare().await.unwrap(); // TODO: error handling

        // TODO: should be biased towards older futures - corner case: consumer taking long time to process messages,
        //       fairness for consuming messages from different brokers
        let x = self
            .mappings
            .iter()
            .map(|(broker_id, mappings)| self.fetch_data(*broker_id, mappings))
            .collect::<Vec<_>>();
        let mut requests_in_flight = Box::pin(select_all(x));

        loop {
            tokio::select! {
                biased;
                None = self.command_receiver.recv() => {
                    break;
                }
                ((broker_id, response),_, mut in_progress) = &mut requests_in_flight => {
                    self.handle_response(broker_id, response).await;
                    in_progress.push(self.fetch_data(broker_id, self.mappings.get(&broker_id).unwrap()));
                    requests_in_flight = Box::pin(select_all(in_progress));
                }
            }
        }

        // TODO: Cleanup waiting requests, leave consumer group etc.

        debug!("Consumer loop is closing");

        // TODO: Make sure it closes when consumer client is dropped - test?
    }

    async fn handle_response(
        &mut self,
        broker_id: i32,
        response: Result<FetchResponse, ApiCallError>,
    ) {
        let response = response.unwrap();
        assert!(response.error_code.is_none());
        let broker_metadata = self.mappings.get_mut(&broker_id).unwrap();
        for topic_response in response.topics {
            let topic_metadata = broker_metadata.get_mut(&topic_response.name).unwrap();
            for partition_response in topic_response.partitions {
                assert!(partition_response.error_code.is_none());
                if let Some(records) = partition_response.records {
                    let mut last_offset = None;
                    let mut remaining_records = BytesMut::from(records.as_slice());
                    while !remaining_records.is_empty() {
                        let batch = RecordBatch::decode(&mut remaining_records);
                        for record in batch.records {
                            let offset = batch.base_offset + record.offset_delta.0 as i64;
                            last_offset = Some(offset);
                            let _ = self
                                .record_sender
                                .send(Record {
                                    timestamp: batch.base_timestamp.add(Duration::from_millis(
                                        record.timestamp_delta.0 as u64,
                                    )),
                                    key: record.key.0,
                                    value: record.value.0,
                                    topic: topic_response.name.clone(),
                                    headers: record
                                        .headers
                                        .0
                                        .into_iter()
                                        .map(|header| (header.key.0, header.value.0))
                                        .collect(),
                                    partition: partition_response.partition_index,
                                    offset,
                                })
                                .await;
                        }
                    }
                    if let Some(x) = last_offset {
                        topic_metadata.insert(partition_response.partition_index, x + 1);
                    }
                }
            }
        }
    }
    fn fetch_data(
        &self,
        broker_id: i32,
        mappings: &PartitionMapping2,
    ) -> Pin<Box<impl Future<Output = (i32, Result<FetchResponse, ApiCallError>)> + use<>>> {
        // TODO: fetch options
        let request = FetchRequest {
            replica_id: -1,
            max_wait: 0,
            min_bytes: 0,
            max_bytes: 52428800, // 50MB, librdkafka defaults
            isolation_level: 1,
            session_id: 0,
            epoch: -1,
            topics: mappings
                .iter()
                .map(|(topic, partitions)| {
                    FetchableTopic {
                        name: topic.to_owned(),
                        fetch_partitions: partitions
                            .iter()
                            .map(|(partition_index, next_offset)| {
                                FetchPartition {
                                    partition_index: *partition_index,
                                    current_leader_epoch: 0, // TODO:
                                    fetch_offset: *next_offset,
                                    log_start_offset: 0, // TODO:
                                    max_bytes: 1048576,  // TODO: 1MB, librdkafka defaults
                                }
                            })
                            .collect(),
                    }
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

    async fn prepare(&self) -> Result<HashMap<i32, PartitionMapping2>, ConsumeError> {
        let mut last_offsets = vec![];

        for (topic, metadata) in self
            .controller
            .get_topic_metadata([self.consumer_options.topic.clone()], ForceRefresh::No)
            .await
            .map_err(MetadataFetchFailed)?
            .into_iter()
        {
            assert!(metadata.error_code.is_none()); // TODO:
            for partition_metadata in metadata.partitions {
                assert!(partition_metadata.error_code.is_none());
                last_offsets.push(PartitionMapping {
                    broker_id: partition_metadata.leader_id,
                    topic: topic.clone(),
                    partition_id: partition_metadata.partition_index,
                });
            }
        }

        last_offsets.sort_unstable_by_key(|x| x.broker_id);

        let list_offset_responses = join_all(
            last_offsets
                .chunk_by(|x, y| x.broker_id == y.broker_id)
                .map(|mappings| {
                    let broker_id = mappings.first().unwrap().broker_id;

                    async move {
                        (
                            broker_id,
                            self.controller
                                .make_api_call(
                                    broker_id,
                                    ListOffsetRequest {
                                        replica_id: -1,
                                        isolation_level: 1, // TODO: Magic number
                                        topics: mappings
                                            .chunk_by(|q1, q2| q1.topic == q2.topic)
                                            .into_iter()
                                            .map(|z| {
                                                let topic = z.first().unwrap().topic.clone();
                                                ListOffsetTopic {
                                                    name: topic,
                                                    partitions: z
                                                        .into_iter()
                                                        .map(|w| {
                                                            ListOffsetPartition {
                                                                partition_index: w.partition_id,
                                                                current_leader_epoch: 0, // TODO:
                                                                timestamp: -2, //earliest // TODO: magic number, value from options
                                                                max_num_offsets: 0,
                                                            }
                                                        })
                                                        .collect(),
                                                }
                                            })
                                            .collect(),
                                    },
                                    None,
                                )
                                .await,
                        )
                    }
                }),
        )
        .await;

        let mut results: HashMap<i32, PartitionMapping2> = HashMap::new();
        for (broker_id, r) in list_offset_responses {
            let r = r.unwrap();
            let b = results.entry(broker_id).or_default();
            for topic in r.topics {
                for partition in topic.partitions {
                    assert!(partition.error_code.is_none());
                    assert_eq!(partition.partition_index, 0);
                    let t = b.entry(topic.name.clone()).or_default();
                    t.insert(partition.partition_index, partition.offset);
                }
            }
        }

        Ok(results)
    }
}
