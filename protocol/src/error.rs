use thiserror::Error as DeriveError;

use crate::api_error::ApiError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("Connected kafka broker doesn't support requested API version. API name: {0} API version: {1} Api field: {2}")]
    OldKafkaVersion(&'static str, i32, &'static str),
    #[error("Kafka API call returned an error {0:?}")]
    KafkaApiError(#[from] ApiError),
}
