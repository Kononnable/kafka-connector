use crate::clients::consumer::client::KafkaConsumerOptions;
use crate::clients::consumer::error::ConsumeError;
use crate::clients::consumer::error::ConsumeError::MetadataFetchFailed;
use crate::clients::consumer::record::Record;
use crate::cluster::controller::{ClusterController, ForceRefresh};
use crate::cluster::error::ApiCallError;
use crate::protocol_consts::{IsolationLevel, ListOffsetsTimestampType};
use bytes::BytesMut;
use futures::future::{join_all, select_all};
use kafka_connector_protocol::fetch_request::{FetchPartition, FetchRequest, FetchableTopic};
use kafka_connector_protocol::fetch_response::{FetchResponse, FetchablePartitionResponse};
use kafka_connector_protocol::list_offset_request::{
    ListOffsetPartition, ListOffsetRequest, ListOffsetTopic,
};
use kafka_connector_protocol::list_offset_response::ListOffsetResponse;
use kafka_connector_protocol::records::record_batch::RecordBatch;
use std::collections::HashMap;
use std::ops::Add;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, instrument};

pub struct ConsumerLoop {
    controller: Arc<ClusterController>,
    consumer_options: KafkaConsumerOptions,
    record_sender: mpsc::Sender<Record>,
    command_receiver: mpsc::Receiver<()>,
    /// Broker -> Topic -> Partition -> next_offset_to_fetch, current_leader_epoch
    mappings: HashMap<i32, HashMap<String, HashMap<i32, (i64, i32)>>>,
    deserialization_buffer: BytesMut,
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
            command_receiver,
            mappings: Default::default(),
            deserialization_buffer: BytesMut::new(),
        }
        .run()
        .await;
    }

    #[instrument(level = "debug", skip(self))]
    async fn run(&mut self) {
        debug!("Consumer loop started.");
        // TODO: Support fetching data from isr replicas
        // TODO: Support metadata change self-healing - from time to time check metadata to see if partition count/broker isr have changed

        self.prepare_mappings().await.unwrap(); // TODO:

        // TODO: should be biased towards older futures - corner case: consumer taking long time to process messages,
        //       fairness for consuming messages from different brokers
        let mut requests_in_flight = Box::pin(select_all(
            self.mappings
                .keys()
                .map(|broker_id| self.fetch_data(*broker_id))
                .collect::<Vec<_>>(),
        ));

        loop {
            tokio::select! {
                biased;
                None = self.command_receiver.recv() => {
                    break;
                }
                ((broker_id, response),_, mut in_progress) = &mut requests_in_flight => {
                    self.handle_response(broker_id, response).await;
                    in_progress.push(self.fetch_data(broker_id));
                    requests_in_flight = Box::pin(select_all(in_progress));
                }
            }
        }

        debug!("Consumer loop is closing");

        // TODO: Make sure it closes when consumer client is dropped - test?
    }

    async fn handle_response(
        &mut self,
        broker: i32,
        response: Result<FetchResponse, ApiCallError>,
    ) {
        let response = response.unwrap(); // TODO:
        assert!(response.error_code.is_none()); // TODO:
        for topic_response in response.topics {
            let topic = topic_response.name.as_str();
            for partition_response in topic_response.partitions {
                assert!(partition_response.error_code.is_none()); // TODO:
                let partition = partition_response.partition_index;
                if let Some(records) = partition_response.records {
                    self.handle_partition(broker, topic, partition, records)
                        .await;
                }
            }
        }
    }

    async fn handle_partition(
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

                let _ = self.record_sender.send(record).await; // TODO: this await may block
            }
        }
        if let Some(offset) = last_processed_offset {
            self.mappings
                .get_mut(&broker)
                .unwrap()
                .get_mut(topic)
                .unwrap()
                .get_mut(&partition)
                .unwrap()
                .0 = offset + 1;
        }
    }

    fn fetch_data(
        &self,
        broker_id: i32,
    ) -> Pin<Box<impl Future<Output = (i32, Result<FetchResponse, ApiCallError>)> + use<>>> {
        let mappings = self.mappings.get(&broker_id).unwrap();
        // TODO: fetch options
        let request = FetchRequest {
            replica_id: -1,
            max_wait: 0,
            min_bytes: 0,
            max_bytes: 52428800, // 50MB, librdkafka defaults
            isolation_level: IsolationLevel::ReadCommited.into(),
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
                                    // TODO: pass leader epoch, handle errors in response - UNKNOWN_LEADER_EPOCH, FENCED_LEADER_EPOCH - force metadata refresh, check if offset exists (records may disappear if unclean leader election is on), send standard fetch requests
                                    current_leader_epoch: 0,
                                    fetch_offset: next_offset.0,
                                    log_start_offset: -1,
                                    max_bytes: 1048576, // TODO: 1MB, librdkafka defaults
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

    async fn prepare_mappings(&mut self) -> Result<(), ConsumeError> {
        assert!(self.mappings.is_empty());

        for (topic, metadata) in self
            .controller
            .get_topic_metadata([self.consumer_options.topic.clone()], ForceRefresh::No)
            .await
            .map_err(MetadataFetchFailed)?
            .into_iter()
        {
            assert!(metadata.error_code.is_none()); // TODO:
            for partition_metadata in metadata.partitions {
                assert!(partition_metadata.error_code.is_none()); // TODO:
                self.mappings
                    .entry(partition_metadata.leader_id)
                    .or_default()
                    .entry(topic.clone())
                    .or_default()
                    .insert(
                        partition_metadata.partition_index,
                        (-1, partition_metadata.leader_epoch),
                    );
            }
        }

        let list_offset_responses = join_all(self.mappings.iter().map(|(broker_id, mappings)| {
            let request = ListOffsetRequest {
                replica_id: -1,
                isolation_level: IsolationLevel::ReadCommited.into(),
                topics: mappings
                    .iter()
                    .map(|(topic, partitions)| {
                        ListOffsetTopic {
                            name: topic.to_owned(),
                            partitions: partitions
                                .iter()
                                .map(|(partition, &(_, current_leader_epoch))| {
                                    ListOffsetPartition {
                                        partition_index: *partition,
                                        current_leader_epoch, // TODO: Handle errors -  UNKNOWN_LEADER_EPOCH, FENCED_LEADER_EPOCH
                                        timestamp: ListOffsetsTimestampType::Earliest.into(), // TODO: value from options
                                        max_num_offsets: 0,
                                    }
                                })
                                .collect(),
                        }
                    })
                    .collect(),
            };

            return send_list_offsets_request(*broker_id, &self.controller, request);

            async fn send_list_offsets_request(
                broker_id: i32,
                controller: &Arc<ClusterController>,
                request: ListOffsetRequest,
            ) -> (i32, Result<ListOffsetResponse, ApiCallError>) {
                (
                    broker_id,
                    controller.make_api_call(broker_id, request, None).await,
                )
            }
        }))
        .await;

        for (broker_id, response) in list_offset_responses {
            let response = response.unwrap(); // TODO:
            let broker_mappings = self.mappings.entry(broker_id).or_default();
            for topic in response.topics {
                for partition in topic.partitions {
                    assert!(partition.error_code.is_none()); // TODO:
                    broker_mappings
                        .entry(topic.name.clone())
                        .or_default()
                        .insert(
                            partition.partition_index,
                            (partition.offset, partition.leader_epoch),
                        );
                }
            }
        }

        Ok(())
    }
}
