use std::{sync::Arc, thread::sleep, time::Duration};

use kafka_connector::{
    broker::options::KafkaClientOptions,
    cluster::Cluster,
    producer::{options::ProducerOptions, record::ProducerRecord, Producer},
};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cluster = Cluster::new(
        "127.0.0.1:9092",
        KafkaClientOptions::builder()
            .client_id("kafka-connector-test".to_owned())
            .build(),
    )
    .await
    .map(Arc::new)
    .unwrap();

    println!("Sleep 2 sec");
    sleep(Duration::from_secs(2));

    let mut producer = Producer::new(cluster, ProducerOptions::builder().build()).await;
    let _ack_promise = producer
        .send(
            ProducerRecord::builder()
                .topic("producer_test".to_owned())
                .payload(b"payload".to_vec())
                .build(),
        )
        .await;
    println!("Sleep 10 sec");
    sleep(Duration::from_secs(10));
    drop(producer);
    println!("Sleep 2 sec");
    sleep(Duration::from_secs(2));
    Ok(())
}
