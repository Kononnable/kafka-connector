use typed_builder::TypedBuilder;

// TODO: Add ability to specify serializers for key, message, header value
#[derive(Clone, TypedBuilder, Default, Debug)]
pub struct ProducerOptions {
    #[builder(default = {1})]
    pub acks: i16,
    // batch.size
    // compression.type
    #[builder(default = {30_000})]
    pub delivery_timeout_ms: i32,
    // enable.idempotence
    // linger.ms
    // max.in.flight.requests.per.connection
    // max.request.size

    // transactions:
    // max.block.ms
    // transaction.timeout.ms
    // transactional.id
}
