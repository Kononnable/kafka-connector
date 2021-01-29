use kafka_connector::kafka_client::KafkaClient;

const BROKER: &str = "127.0.0.1:9092";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let kafka_client = KafkaClient::new(BROKER).await?;
    let broker_client = kafka_client.clients.first().unwrap();

    Ok(())
}
