use anyhow::Result;

use crate::kafka_client::KafkaClient;

const BROKER: &str = "127.0.0.1:9092";
const CLIENT_ID: &str = "kafka-connector-test";

#[tokio::test]
async fn should_connect_to_kafka() -> Result<()> {
    env_logger::init();
    KafkaClient::new(BROKER, CLIENT_ID).await?;
    Ok(())
}
#[tokio::test]
async fn should_fetch_api_versions_after_connect() -> Result<()> {
    let kafka_client = KafkaClient::new(BROKER, CLIENT_ID).await?;
    let broker_client = kafka_client.clients.first().unwrap();

    assert!(!broker_client.supported_versions.is_empty());
    Ok(())
}
