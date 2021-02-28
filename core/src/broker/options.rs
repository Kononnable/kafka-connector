use derive_builder::Builder;

#[derive(Clone, Builder, Default, Debug)]
pub struct KafkaClientOptions {
    // client.id
    // connections.max.idle.ms
    // metadata.max.age.ms
    // reconnect.backoff.max.ms
    // reconnect.backoff.ms
    // request.timeout.ms
    // retry.backoff.ms
    // socket.connection.setup.timeout.max.ms
    #[builder(default = {10_000})]
    pub socket_connection_setup_timeout_ms: u32,
}
