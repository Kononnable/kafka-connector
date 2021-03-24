use crate::broker::options::KafkaClientOptions;

use self::{
    error::ProducerCreateError, options::ProducerOptions, record::ProducerRecord,
    send_result_future::SendResultFuture,
};

pub mod error;
pub mod options;
pub mod record;
pub mod send_result_future;

pub struct Producer {}

impl Producer {
    /// Creates kafka producer and waits for it to connect to at least one broker
    pub async fn new(
        _client_options: KafkaClientOptions,
        _producer_options: ProducerOptions,
    ) -> Result<Self, ProducerCreateError> {
        todo!()
    }
    pub fn send(&mut self, _record: ProducerRecord<'_>) -> SendResultFuture {
        todo!()
    }
    pub fn commit_offsets() {
        todo!()
    }
    pub fn store_offset() {
        todo!()
    }
}
