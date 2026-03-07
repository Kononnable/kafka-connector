use crate::clients::consumer::error::ConsumeError;
use crate::clients::consumer::error::ConsumeError::MetadataFetchFailed;
use crate::clients::consumer::options::{KafkaConsumerOptions, OffsetReset};
use crate::clients::consumer::record::Record;
use crate::cluster::controller::{ClusterController, ForceRefresh};
use crate::cluster::error::ApiCallError;
use crate::protocol_consts::IsolationLevel;
use bytes::BytesMut;
use futures::future::{join_all, select_all};
use kafka_connector_protocol::fetch_request::{FetchPartition, FetchRequest, FetchableTopic};
use kafka_connector_protocol::fetch_response::FetchResponse;
use kafka_connector_protocol::list_offset_request::{
    ListOffsetPartition, ListOffsetRequest, ListOffsetTopic,
};
use kafka_connector_protocol::metadata_request::MetadataRequest;
use kafka_connector_protocol::records::record_batch::RecordBatch;
use kafka_connector_protocol::{ApiError, ApiRequest};
use std::collections::HashMap;
use std::ops::Add;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, instrument, warn};

/// Broker -> Topic -> Partition -> (next_offset_to_fetch, current_leader_epoch)
type Mappings = HashMap<i32, HashMap<String, HashMap<i32, (i64, i32)>>>;

pub struct ConsumerLoop {
    controller: Arc<ClusterController>,
    consumer_options: KafkaConsumerOptions,
    record_sender: mpsc::Sender<Record>,
    /// Broker -> Topic -> Partition -> (next_offset_to_fetch, current_leader_epoch)
    mappings: Mappings,
    deserialization_buffer: BytesMut,
    reinitialize_triggered: bool,
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
            mappings: Default::default(),
            deserialization_buffer: BytesMut::new(),
            reinitialize_triggered: false,
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
        let mut requests_in_flight = None;
        loop {
            if requests_in_flight.is_none() {
                let refresh = if self.reinitialize_triggered {
                    ForceRefresh::No
                } else {
                    ForceRefresh::Yes
                };
                self.reinitialize_triggered = false;
                if let Err(err) = self.sync_metadata(refresh).await {
                    self.reinitialize_triggered = true;
                    info!(
                        "Encountered {err} during consumer metadata synchronization, retrying in 1000 ms."
                    );
                    tokio::time::sleep(Duration::from_millis(1_000)).await;
                }

                // Can be triggered by sync_metadata internally, or by returning an error
                if self.reinitialize_triggered {
                    continue;
                }

                requests_in_flight = Some(Box::pin(select_all(
                    self.mappings
                        .keys()
                        .map(|broker_id| self.fetch_data(*broker_id))
                        .collect::<Vec<_>>(),
                )));
            }

            let ((broker_id, response), _, in_progress) = requests_in_flight.take().unwrap().await;
            self.process_fetch_response(broker_id, response).await;

            if self.reinitialize_triggered {
                if !in_progress.is_empty() {
                    requests_in_flight = Some(Box::pin(select_all(in_progress)));
                }
                continue;
            }

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
            requests_in_flight = Some(Box::pin(select_all(new_in_progress)));
        }
    }

    #[instrument(level = "debug", skip(self))]
    async fn sync_metadata(&mut self, refresh: ForceRefresh) -> Result<(), ConsumeError> {
        for (topic, topic_metadata) in self
            .controller
            .get_metadata(self.consumer_options.topics.clone(), refresh)
            .await
            .map_err(MetadataFetchFailed)?
            .into_iter()
        {
            if let Some(error_code) = topic_metadata.error_code {
                match error_code {
                    ApiError::UnknownTopicOrPartition => {
                        // remove old mappings if topic was removed
                        self.mappings.iter_mut().for_each(|(_, mappings)| {
                            mappings.remove(&topic);
                        });
                        warn!("Kafka Topic {} not found", topic);
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

                let (_, epoch) = self
                    .mappings
                    .entry(partition_metadata.leader_id)
                    .or_default()
                    .entry(topic.clone())
                    .or_default()
                    .entry(partition_metadata.partition_index)
                    .or_insert((-1, -1));
                *epoch = partition_metadata.leader_epoch;
            }
        }

        if self.mappings.is_empty() {
            return Err(ConsumeError::NoValidTopicsFound());
        }

        let list_offset_responses = join_all(self.mappings.iter().map(|(broker_id, mappings)| {
            let request = ListOffsetRequest {
                replica_id: -1,
                isolation_level: IsolationLevel::ReadCommited.into(),
                topics: mappings
                    .iter()
                    .map(|(topic, partitions)| ListOffsetTopic {
                        name: topic.to_owned(),
                        partitions: partitions
                            .iter()
                            .map(|(&partition_index, &(_, current_leader_epoch))| {
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
            async move {
                (
                    *broker_id,
                    controller.make_api_call(*broker_id, request, None).await,
                )
            }
        }))
        .await;

        for (broker_id, response) in list_offset_responses {
            let broker_mappings = self.mappings.entry(broker_id).or_default();
            for topic in response?.topics {
                for partition in topic.partitions {
                    if let Some(error_code) = partition.error_code {
                        match error_code {
                            ApiError::UnknownLeaderEpoch | ApiError::FencedLeaderEpoch => {
                                self.reinitialize_triggered = true;
                                return Ok(());
                            }
                            _ => Err(ApiCallError::UnexpectedErrorCode(
                                ListOffsetRequest::get_api_key(),
                                error_code,
                                "topics.partitions.error_code",
                            ))?,
                        }
                    }
                    let (offset, _) = broker_mappings
                        .entry(topic.name.clone())
                        .or_default()
                        .entry(partition.partition_index)
                        .or_insert((-1, partition.leader_epoch));
                    if *offset == -1
                        || (*offset < partition.offset
                            && self.consumer_options.offset_reset == OffsetReset::Earliest)
                    {
                        // TODO: make sure corer case works - timestamp !=earliest and consumer fallen behind retention (data deleted)
                        *offset = partition.offset;
                    }
                }
            }
        }

        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    fn fetch_data(
        &self,
        broker_id: i32,
    ) -> Pin<Box<impl Future<Output = (i32, Result<FetchResponse, ApiCallError>)> + use<>>> {
        let mappings = self.mappings.get(&broker_id).unwrap();
        let request = FetchRequest {
            replica_id: -1,
            max_wait: self.consumer_options.max_wait.as_millis() as i32,
            min_bytes: self.consumer_options.min_bytes,
            max_bytes: self.consumer_options.max_bytes,
            isolation_level: IsolationLevel::ReadCommited.into(),
            session_id: 0,
            epoch: -1,
            topics: mappings
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
                "topics.partitions.error_code",
            );
            info!("Encountered {err} during records consumption.");
            self.reinitialize_triggered = true;
        }
        for topic_response in response.topics {
            let topic = topic_response.name.as_str();
            for partition_response in topic_response.partitions {
                if let Some(error_code) = response.error_code {
                    match error_code {
                        ApiError::UnknownTopicOrPartition
                        | ApiError::NotLeaderOrFollower
                        | ApiError::ReplicaNotAvailable
                        | ApiError::FencedLeaderEpoch
                        | ApiError::UnknownLeaderEpoch
                        | ApiError::OffsetOutOfRange => self.reinitialize_triggered = true,
                        ApiError::RequestTimedOut => {}
                        _ => {
                            let err = ApiCallError::UnexpectedErrorCode(
                                FetchRequest::get_api_key(),
                                error_code,
                                "topics.partitions.error_code",
                            );
                            info!("Encountered {err} during records consumption.");
                            self.reinitialize_triggered = true;
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

                let _ = self.record_sender.send(record).await;
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
}
