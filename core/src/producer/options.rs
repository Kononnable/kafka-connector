use typed_builder::TypedBuilder;

// TODO: Add ability to specify serializers for key, message, header value
#[derive(Clone, TypedBuilder, Default, Debug)]
pub struct ProducerOptions {
    // acks
// batch.size
// compression.type
// delivery.timeout.ms
// enable.idempotence
// linger.ms
// max.in.flight.requests.per.connection
// max.request.size

// transactions:
// max.block.ms
// transaction.timeout.ms
// transactional.id
}
