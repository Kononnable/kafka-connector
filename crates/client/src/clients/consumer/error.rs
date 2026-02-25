use crate::cluster::error::ApiCallError;
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ConsumeError {
    #[error("No topics found to subscribe to")]
    NoValidTopicsFound(),
    #[error("Metadata fetch request failed: {0}")]
    MetadataFetchFailed(ApiCallError),
    #[error("Kafka API returned unexpected error {0}")]
    ApiCallError(#[from] ApiCallError),
}
