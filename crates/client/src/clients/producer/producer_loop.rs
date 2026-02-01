use crate::clients::producer::client::{KafkaProducerOptions, ProduceRequestMessage};
use crate::cluster::controller::{ClusterController, ForceRefresh};
use bytes::BytesMut;
use kafka_connector_protocol::produce_request::{
    PartitionProduceData, ProduceRequest, TopicProduceData,
};
use kafka_connector_protocol::records::base_types::{
    VarIntBytes, VarIntString, VarIntVec, VarLong,
};
use kafka_connector_protocol::records::header::Header;
use kafka_connector_protocol::records::record::Record;
use kafka_connector_protocol::records::record_batch::RecordBatch;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::mpsc;
use tracing::{debug, instrument};

pub struct ProducerLoop {
    controller: Arc<ClusterController>,
    producer_options: KafkaProducerOptions,
    receiver: mpsc::UnboundedReceiver<ProduceRequestMessage>,
    serialization_buffer: BytesMut,
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
            receiver,
            serialization_buffer: BytesMut::with_capacity(10_000_000), // TODO: size from options, at least 2 times max record_batch, default same as for rdkafka
        }
        .run()
        .await;
    }

    #[instrument(level = "debug", skip(self))]
    async fn run(&mut self) {
        debug!("Producer loop started.");
        // TODO: Add batching based on time
        // TODO: Add concurrency (loop with select! and queues) - nonblocking

        loop {
            let signal = self.receiver.recv().await;
            match signal {
                None => {
                    break;
                }
                Some(signal) => {
                    let default_timestamp = SystemTime::now();

                    // TODO: Clean, error handling, remove const values

                    let sig = signal.records.into_iter().next().unwrap();

                    let metadata = self
                        .controller
                        .get_topic_metadata(&[&sig.topic], ForceRefresh::No)
                        .await
                        .unwrap()
                        .into_values()
                        .next()
                        .unwrap();

                    assert!(metadata.error_code.is_none());
                    let partition_metadata = metadata.partitions.first().unwrap();
                    assert!(partition_metadata.error_code.is_none());
                    assert_eq!(partition_metadata.partition_index, 0);
                    let broker_id = partition_metadata.leader_id;

                    let mut record_batch = RecordBatch {
                        records: vec![Record {
                            timestamp_delta: VarLong(
                                sig.timestamp
                                    .unwrap_or(default_timestamp)
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as i64,
                            ),
                            key: VarIntBytes(sig.key),
                            value: VarIntBytes(sig.value),
                            headers: VarIntVec(
                                sig.headers
                                    .into_iter()
                                    .map(|(k, v)| Header {
                                        header_key: VarIntString(k),
                                        value: VarIntBytes(v),
                                    })
                                    .collect(),
                            ),
                            ..Default::default()
                        }],
                        ..Default::default()
                    };
                    record_batch.encode(&mut self.serialization_buffer);
                    let records = self.serialization_buffer.to_vec();
                    self.serialization_buffer.clear();

                    let resp = self
                        .controller
                        .make_api_call(
                            broker_id,
                            ProduceRequest {
                                acks: 1,
                                topics: vec![TopicProduceData {
                                    name: sig.topic.clone(),
                                    partitions: vec![PartitionProduceData {
                                        partition_index: 0,
                                        records: Some(records),
                                    }],
                                }],
                                ..Default::default()
                            },
                            None,
                        )
                        .await
                        .unwrap();

                    for resp in resp.responses {
                        for part in resp.partitions {
                            assert!(part.error_code.is_none());
                        }
                    }
                    signal.response_sender.send(()).unwrap();
                }
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

        // TODO: Cleanup waiting/in-flight requests

        debug!("Producer loop is closing");

        // TODO: Make sure it closes when broker is dropped - test?
    }
}
