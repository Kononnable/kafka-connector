use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ClusterControllerCreationError {
    // #[error("Failed to connect to kafka cluster [{}]", .0.iter().fold(String::new(), | acc, num | acc + & num.to_string() + ", "))]
    // ConnectionError(Vec<KafkaApiCallError>),
    // #[error("Failed to recognize cluster address {0}")]
    // AddressRecognitionFailed(#[from] std::io::Error),
    // #[error("No cluster address provided")]
    // NoClusterAddressFound(),
    // #[error("Failed to fetch broker list {0}")]
    // MetadataFetchFailed(#[from] KafkaApiCallError),
}
