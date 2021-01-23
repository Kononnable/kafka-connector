use crate::{kafka_client::KafkaClient, producer_options::ProducerOptions};

pub struct Producer {}

impl Producer {
    pub fn new(_kafka_client: KafkaClient, _options: ProducerOptions) -> Self {
        Self {}
    }
}
