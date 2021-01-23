use anyhow::Result;

use crate::kafka_client::KafkaClient;

const BROKER: &str  = "127.0.0.1:9092";

#[tokio::test] 
async fn should_connect_to_kafka()->Result<()> {
    KafkaClient::new(BROKER).await?;
    Ok(())
}
#[tokio::test] 
async fn should_fetch_api_versions_after_connect()->Result<()> {
    let kafka_client = KafkaClient::new(BROKER).await?;
    let broker_client = kafka_client.clients.first().unwrap();

    assert!(!broker_client.supported_versions.is_empty());
    Ok(())
}