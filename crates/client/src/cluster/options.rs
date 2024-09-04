use std::time::Duration;

// TODO: Rename fields?
// TODO: add all options (from 1.0, without ones introduced later in KIPs to be implemented)
#[derive(Clone, Debug)]
pub struct ClusterControllerOptions {
    pub connection_retires: u8,
    pub connection_retry_delay: Duration,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub client_name: String,
}

// TODO: Set proper defaults
// TODO: Use derivative or something similar? (for more readability)
impl Default for ClusterControllerOptions {
    fn default() -> Self {
        ClusterControllerOptions {
            connection_retires: 5,
            connection_retry_delay: Duration::from_millis(1_000),
            connection_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(5),
            client_name: "kafka-connector".to_owned(),
        }
    }
}
