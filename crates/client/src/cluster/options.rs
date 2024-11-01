use derivative::Derivative;
use std::time::Duration;

// TODO: Rename fields?
// TODO: add all options (from 1.0, without ones introduced later in KIPs to be implemented)
// TODO: Set proper defaults
#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct ClusterControllerOptions {
    #[derivative(Default(value = "5"))]
    pub connection_retires: u8,
    #[derivative(Default(value = "Duration::from_millis(1_000)"))]
    pub connection_retry_delay: Duration,
    #[derivative(Default(value = "Duration::from_millis(5_000)"))]
    pub connection_timeout: Duration,
    #[derivative(Default(value = "Duration::from_millis(5_000)"))]
    pub request_timeout: Duration,
    #[derivative(Default(value = "\"kafka-connector\".to_owned()"))]
    pub client_name: String,
}
