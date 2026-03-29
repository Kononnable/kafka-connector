use kafka_connector_protocol::{ApiError, ApiKey};
use std::sync::Arc;
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Clone, Debug, DeriveError)]
pub enum ApiCallError {
    #[error("Broker connection closed before api response was received")]
    BrokerConnectionClosed,
    #[error("No broker with id {0} found")]
    BrokerNotFound(i32),
    #[error("Error encountered during network communication. {0}")]
    IoError(Arc<std::io::Error>),
    #[error("Serialization error {0}")]
    SerializationError(#[from] kafka_connector_protocol::SerializationError),
    #[error("ApiCall timeout reached")]
    TimeoutReached,
    #[error("Broker does not support API {0:?}")]
    UnsupportedApi(ApiKey),
    #[error("Broker returned unexpected error code {1:?} in field {2} when calling API {0:?} ")]
    UnexpectedErrorCode(ApiKey, ApiError, &'static str),
}

impl From<std::io::Error> for ApiCallError {
    fn from(value: std::io::Error) -> Self {
        ApiCallError::IoError(Arc::new(value))
    }
}
