use crate::cluster::error::ApiCallError;
use kafka_connector_protocol::ApiError;
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ConsumeError {
    #[error("No topics found to subscribe to")]
    NoValidTopicsFound(),
    #[error("Metadata fetch request failed: {0}")]
    MetadataFetchFailed(ApiCallError),
    #[error("Partition leader was fenced. Topic: {0}, Partition: {1}")]
    PartitionLeaderFenced(String, i32),
    #[error("Kafka API returned unexpected error {0}")]
    ApiCallError(#[from] ApiCallError),
    #[error("Consumer group coordinator is temporarily unavailable. {0:?}")]
    CoordinatorNotReady(ApiError),
    #[error("Consumer group authorization failed. Group Id: {group_id}, Error: {error:?}")]
    GroupAuthorizationFailed {
        group_id: String,
        error: Option<String>,
    },
}

impl ConsumeError {
    pub fn is_transisient(&self) -> bool {
        matches!(
            self,
            ConsumeError::PartitionLeaderFenced(_, _) | ConsumeError::CoordinatorNotReady(_)
        )
    }
}
