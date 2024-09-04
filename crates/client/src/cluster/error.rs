use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ClusterControllerCreationError {
    #[error("Bootstrap address list is empty")]
    NoClusterAddressFound,
    #[error("No connection established within {0} attempts")]
    OutOfConnectionAttempts(u16),
}
