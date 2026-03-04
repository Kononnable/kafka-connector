use derivative::Derivative;
use std::time::Duration;

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct ClusterControllerOptions {
    /// Number of retries during Cluster Controller creation before ClusterController::new() returns error.
    #[derivative(Default(value = "5"))]
    pub initialization_retires: u8,

    /// Delay before retrying after connection failure.
    #[derivative(Default(value = "Duration::from_millis(1_000)"))]
    pub connection_retry_delay: Duration,

    /// Duration after which connection attempt is treated as failed.
    #[derivative(Default(value = "Duration::from_millis(30_000)"))]
    pub connection_timeout: Duration,

    /// Timeout for kafka api calls
    #[derivative(Default(value = "Duration::from_secs(30_000)"))]
    pub request_timeout: Duration,

    /// Client identifier
    #[derivative(Default(value = "env!(\"CARGO_CRATE_NAME\").to_owned()"))]
    pub client_name: String,

    /// Advanced settings, change only if needed and you understand the consequences
    pub advanced: ClusterControllerAdvancedOptions,
}

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct ClusterControllerAdvancedOptions {
    /// Limit of parallel api requests for single broker connection
    #[derivative(Default(value = "5"))]
    pub max_requests_per_connection: usize,

    /// Initial buffer size for serializing/deserializing kafka api messages.
    #[derivative(Default(value = "2 * 1024 * 1024"))] // 2MB
    pub buffer_size: usize,

    /// Amount of bytes buffers grow when they're near limit.
    ///
    /// Note: Not every operation checks if buffer is near full.
    /// Checking and increasing the buffer size can be resource intensive.
    #[derivative(Default(value = "1024 * 1024"))] // 1MB
    pub buffer_grow_size: usize,

    /// Maximum time for which metadata is kept in cache before considered outdated.
    #[derivative(Default(value = "Duration::from_millis(300_000)"))]
    pub metadata_refresh_interval: Duration,
}
