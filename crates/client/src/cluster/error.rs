use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ClusterControllerCreationError {
    // #[error("Failed to connect to kafka cluster [{}]", .0.iter().fold(String::new(), | acc, num | acc + & num.to_string() + ", "))]
    // ConnectionError(Vec<KafkaApiCallError>),
    // #[error("Failed to recognize cluster address {0}")]
    // AddressRecognitionFailed(#[from] std::io::Error),
    #[error("Bootstrap address list is empty")]
    NoClusterAddressFound,
    #[error("No connection established within {0} attempts")]
    OutOfConnectionAttempts(u16),
    // #[error("Failed to fetch broker list {0}")]
    // MetadataFetchFailed(#[from] KafkaApiCallError),
}

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub(super) enum ClusterControllerInitializationError {
    #[error("Error encountered during network communication. {0}")]
    NetworkError(std::io::Error),
    #[error("Broker did not respond within specified time")]
    ApiCallTimeoutReached,
    #[error("Failed to connect with the broker. {0}")]
    ConnectionError(std::io::Error),
    #[error("Connection not established within specified time")]
    ConnectionTimeoutReached,
}
