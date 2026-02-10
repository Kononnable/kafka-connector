use crate::cluster::error::ApiCallError;
use std::sync::Arc;
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ProduceError {
    #[error("Kafka topic {0} does not exist")]
    TopicNotFound(String),
    #[error("Topic {0} does not have partition {1}")]
    PartitionNotFound(String, i32),
    #[error("Metadata fetch request failed: {0}")]
    MetadataFetchFailed(Arc<ApiCallError>),
    #[error("Kafka API returned an error {0}")]
    ApiCallError(#[from] ApiCallError),
}
