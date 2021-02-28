use derive_builder::Builder;

use crate::broker::options::KafkaClientOptions;

// TODO: Add ability to specify serializers for key, message, header value
#[derive(Clone, Builder, Default, Debug)]
pub struct ProducerOptions {
    pub general_options: KafkaClientOptions,
    pub producer_options: ProducerSpecificOptions,
}

#[derive(Clone, Builder, Default, Debug)]
pub struct ProducerSpecificOptions {
    // acks
// batch.size
// compression.type
// delivery.timeout.ms
// enable.idempotence
// linger.ms
// max.in.flight.requests.per.connection
// max.request.size
// metadata.max.idle.ms
// retries

// transactions:
// max.block.ms
// transaction.timeout.ms
// transactional.id
}
