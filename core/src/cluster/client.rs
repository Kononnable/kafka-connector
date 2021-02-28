use crate::broker::{client::BrokerClient, error::KafkaApiCallError};

use super::metadata::Metadata;

#[derive(Debug)]
pub struct ClusterClient {
    pub client_id: String,
    pub clients: Vec<BrokerClient>,
    pub metadata: Metadata,
}

impl ClusterClient {
    pub async fn new(
        broker_addr: &str,
        client_id: &str,
    ) -> Result<ClusterClient, KafkaApiCallError> {
        let clients = vec![BrokerClient::new(broker_addr, client_id.to_owned()).await?];
        Ok(ClusterClient {
            clients,
            metadata: Metadata::default(),
            client_id: client_id.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    const BROKER: &str = "127.0.0.1:9092";
    const CLIENT_ID: &str = "kafka-connector-test";

    #[tokio::test]
    async fn should_connect_to_kafka() -> Result<()> {
        env_logger::init();
        ClusterClient::new(BROKER, CLIENT_ID).await?;
        Ok(())
    }
    #[tokio::test]
    async fn should_fetch_api_versions_after_connect() -> Result<()> {
        let kafka_client = ClusterClient::new(BROKER, CLIENT_ID).await?;
        let broker_client = kafka_client.clients.first().unwrap();

        assert!(!broker_client.supported_versions.is_empty());
        Ok(())
    }
}
