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
    BrokerConnectionClosing, // TODO: Closing or closed?
    #[error("No broker with id {0} found")]
    BrokerNotFound(i32),
    #[error("Error encountered during network communication. {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error {0}")]
    SerializationError(#[from] kafka_connector_protocol::SerializationError),
}
