use kafka_connector_protocol::{api::ApiNumbers, api_error::ApiError};
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum KafkaApiCallError {
    #[error("Error connecting to broker \"{0}\"")]
    ConnectionError(#[from] std::io::Error),
    #[error("Kafka broker doesn't support requested API version. API: {api:?} version: {version}")]
    OldKafkaVersion { api: ApiNumbers, version: i16 },
    #[error("Kafka API call returned an error {0:?}")]
    KafkaApiError(#[from] ApiError),
}
