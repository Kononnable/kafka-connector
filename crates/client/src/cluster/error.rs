use kafka_connector_protocol::{ApiError, ApiKey};
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ClusterControllerCreationError {
    #[error("Bootstrap address list is empty")]
    NoClusterAddressFound,
    #[error("No connection established within {0} attempts")]
    OutOfConnectionAttempts(u16),
}

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ApiCallError {
    #[error("Broker connection closed before api response was received")]
    BrokerConnectionClosed,
    #[error("No broker with id {0} found")]
    BrokerNotFound(i32),
    #[error("Error encountered during network communication. {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error {0}")]
    SerializationError(#[from] kafka_connector_protocol::SerializationError),
    #[error("ApiCall timeout reached")]
    TimeoutReached,
    #[error("Broker does not support API {0:?}")]
    UnsupportedApi(ApiKey),
    #[error("Broker returned unsupported error code {1:?} in field {2} when calling API {0:?} ")]
    UnsupportedErrorCode(ApiKey, ApiError, &'static str),
}
