use std::{sync::Arc, thread::sleep, time::Duration};

use futures::TryStreamExt;
use kafka_connector::{
    broker::options::KafkaClientOptions,
    cluster::Cluster,
    consumer::{options::ConsumerOptions, Consumer},
};
use tokio::pin;

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

    let consumer = Consumer::new(
        cluster,
        ConsumerOptions::builder()
            .group_id("kafka-connector-test".to_owned())
            .topics(vec!["producer_test".to_owned()])
            .build(),
    )
    .await;

    println!("Sleep 10 sec");
    sleep(Duration::from_secs(10));

    let message_stream = consumer.stream().await.unwrap();
    pin!(message_stream);

    let x = message_stream.try_next().await.unwrap();
    println!("{:?}", x);
    drop(message_stream);
    println!("Sleep 2 sec");
    sleep(Duration::from_secs(2));
    Ok(())
}
