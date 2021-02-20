use crate::kafka_client::KafkaClient;

use self::options::ProducerOptions;

pub mod options;

pub struct Producer {}

impl Producer {
    pub fn new(_kafka_client: KafkaClient, _options: ProducerOptions) -> Self {
        Self {}
    }
}
