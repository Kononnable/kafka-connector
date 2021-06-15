use std::time::Duration;

use typed_builder::TypedBuilder;

#[derive(Clone, TypedBuilder, Default, Debug)]
pub struct KafkaClientOptions {
    #[builder(default = {Duration::from_secs(300)})]
    pub metadata_refresh_timeout: Duration,

    #[builder(default = {4096})]
    pub buffer_size: usize,
    pub client_id: String,
    // client.id
    // connections.max.idle.ms
    // metadata.max.age.ms
    // reconnect.backoff.max.ms
    // reconnect.backoff.ms
    // request.timeout.ms
    #[builder(default = {Duration::from_millis(100)})]
    pub retry_backoff_ms: Duration,
    // socket.connection.setup.timeout.max.ms
    #[builder(default = {u32::MAX})]
    pub retries: u32,
    // #[builder(default = {10_000})]
    // pub socket_connection_setup_timeout_ms: u32,
}
