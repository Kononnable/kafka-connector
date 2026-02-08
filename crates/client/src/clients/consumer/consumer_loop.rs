use crate::clients::consumer::client::KafkaConsumerOptions;
use crate::cluster::controller::ClusterController;
use bytes::BytesMut;
use kafka_connector_protocol::fetch_request::{FetchPartition, FetchRequest, FetchableTopic};
use kafka_connector_protocol::list_offset_request::{
    ListOffsetPartition, ListOffsetRequest, ListOffsetTopic,
};
use kafka_connector_protocol::metadata_request::{MetadataRequest, MetadataRequestTopic};
use kafka_connector_protocol::metadata_response::MetadataResponseTopicKey;
use kafka_connector_protocol::records::record_batch::RecordBatch;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, instrument};

pub struct ConsumerLoop {
    controller: Arc<ClusterController>,
    consumer_options: KafkaConsumerOptions,
    record_sender: mpsc::Sender<(Vec<u8>, Vec<u8>)>,
}
impl ConsumerLoop {
    #[instrument(level = "debug", skip_all)]
    pub async fn start(
        controller: Arc<ClusterController>,
        consumer_options: KafkaConsumerOptions,
        record_sender: mpsc::Sender<(Vec<u8>, Vec<u8>)>,
    ) {
        ConsumerLoop {
            controller,
            consumer_options,
            record_sender,
        }
        .run()
        .await;
    }

    #[instrument(level = "debug", skip(self))]
    async fn run(&mut self) {
        debug!("Consumer loop started.");
        // TODO: Proper loop, consumer groups etc.

        let metadata = self
            .controller
            .make_api_call(
                1,
                MetadataRequest {
                    topics: Some(vec![MetadataRequestTopic {
                        name: self.consumer_options.topic.clone(),
                    }]),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let metadata = metadata
            .topics
            .get(&MetadataResponseTopicKey {
                name: self.consumer_options.topic.clone(),
            })
            .unwrap();

        assert!(metadata.error_code.is_none());
        let partition_metadata = metadata.partitions.first().unwrap();
        assert!(partition_metadata.error_code.is_none());
        // TODO: Single topic partition support for now
        let broker_id = partition_metadata.leader_id;

        let list_offsets = self
            .controller
            .make_api_call(
                broker_id,
                ListOffsetRequest {
                    replica_id: -1,
                    isolation_level: 1,
                    topics: vec![ListOffsetTopic {
                        name: self.consumer_options.topic.clone(),
                        partitions: vec![ListOffsetPartition {
                            partition_index: 0,
                            current_leader_epoch: 0,
                            timestamp: -2, //earliest
                            max_num_offsets: 0,
                        }],
                    }],
                },
                None,
            )
            .await
            .unwrap();
        assert!(
            list_offsets
                .topics
                .first()
                .unwrap()
                .partitions
                .first()
                .unwrap()
                .error_code
                .is_none()
        );
        assert_eq!(
            list_offsets
                .topics
                .first()
                .unwrap()
                .partitions
                .first()
                .unwrap()
                .partition_index,
            0
        );
        let mut next_offset = list_offsets
            .topics
            .first()
            .unwrap()
            .partitions
            .first()
            .unwrap()
            .offset;

        for _ in 0..10 {
            // TODO: Proper loop, current for easy testability
            let fetch_response = self
                .controller
                .make_api_call(
                    broker_id,
                    FetchRequest {
                        replica_id: -1,
                        max_wait: 0,
                        min_bytes: 0,
                        max_bytes: 52428800, // 50MB, librdkafka defaults
                        isolation_level: 1,
                        session_id: 0,
                        epoch: -1,
                        topics: vec![FetchableTopic {
                            name: self.consumer_options.topic.clone(),
                            fetch_partitions: vec![FetchPartition {
                                partition_index: 0,
                                current_leader_epoch: 0,
                                fetch_offset: next_offset,
                                log_start_offset: 0,
                                max_bytes: 1048576, // 1MB, librdkafka defaults
                            }],
                        }],
                        forgotten: vec![],
                    },
                    None,
                )
                .await
                .unwrap();
            assert!(fetch_response.error_code.is_none());
            let partition_data = fetch_response
                .topics
                .first()
                .unwrap()
                .partitions
                .first()
                .unwrap();
            assert!(partition_data.error_code.is_none());
            let mut remaining_records = BytesMut::new();
            remaining_records.extend_from_slice(partition_data.records.as_ref().unwrap());
            while !remaining_records.is_empty() {
                let batch = RecordBatch::decode(&mut remaining_records);
                for record in batch.records {
                    self.record_sender
                        .send((record.key.0, record.value.0))
                        .await
                        .unwrap();
                }
                next_offset = batch.base_offset + batch.last_offset_delta as i64 + 1;
            }
        }
        // loop {
        //     tokio::select! {
        //         signal = self.receiver.recv() => {
        //             match signal {
        //                 Some(request) => {
        //                     //TODO:
        //                 }
        //                 None => { break;}
        //             }
        //         }
        //     }
        // }

        // TODO: Cleanup waiting requests, leave consumer group etc.

        debug!("Consumer loop is closing");

        // TODO: Make sure it closes when consumer client is dropped - test?
    }
}
