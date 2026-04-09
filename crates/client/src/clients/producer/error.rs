use crate::cluster::error::ApiCallError;
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError, Clone)]
pub enum ProduceError {
    #[error("Kafka topic {0} does not exist")]
    TopicNotFound(String),
    #[error("Topic {0} does not have partition {1}")]
    PartitionNotFound(String, i32),
    #[error("Metadata fetch request failed: {0}")]
    MetadataFetchFailed(ApiCallError),
    #[error("Kafka API returned unexpected error {0}")]
    ApiCallError(ApiCallError),
    #[error("Kafka producer closed before message was delivered")]
    ProducerClosed,
}
