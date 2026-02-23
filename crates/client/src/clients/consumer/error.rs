use crate::cluster::error::ApiCallError;
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ConsumeError {
    #[error("Kafka topic {0} does not exist")]
    TopicNotFound(String),
    #[error("Metadata fetch request failed: {0}")]
    MetadataFetchFailed(ApiCallError),
    // #[error("Kafka API returned an error {0}")]
    // ApiCallError(#[from] ApiCallError),
}
