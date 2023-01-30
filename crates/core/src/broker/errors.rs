use std::io;

use kafka_connector_protocol::api_error::ApiError;
use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ConnectionError {
    #[error("Broker address was not specified.")]
    NoAddressSpecified,
    #[error("Broker is already connected.")]
    AlreadyConnected,
    #[error("Error connecting to the broker: {0:?}")]
    NetworkError(#[from] io::Error),
    #[error("Unsupported kafka version. Broker version < 0.10 ")]
    UnsupportedVersion,
    #[error("Unknown api error encountered while fetching supported api versions {0:?}")]
    UnknownError(#[from] ApiError), // TODO: Remove?
}
