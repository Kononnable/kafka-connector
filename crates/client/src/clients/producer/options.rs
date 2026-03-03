use crate::clients::producer::partitioner::{Partitioner, crc32::Crc32Partitioner};
use derivative::Derivative;
use std::time::Duration;

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct KafkaProducerOptions<P = Crc32Partitioner>
where
    P: Partitioner,
{
    /// Acknowledgements from brokers needed for a message to be successfully delivered.
    #[derivative(Default(value = "Acks::All"))]
    pub acks: Acks,

    /// Maximum time for messages to be acknowledged, before they are treated as failed and retry is triggered.
    #[derivative(Default(value = "None"))]
    pub timeout: Option<Duration>, // TODO: Test

    /// Partitioner that will be used for assigning messages to specific partitions
    pub partitioner: P,
}

impl KafkaProducerOptions<Crc32Partitioner> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Acks {
    /// No broker acknowledge messages
    NoAck, // TODO: Implement; tests
    /// Partition leaders acknowledge messages
    Leader,
    /// All in-sync replicas acknowledge messages
    All,
}

impl From<Acks> for i16 {
    fn from(value: Acks) -> Self {
        match value {
            Acks::NoAck => 0,
            Acks::Leader => 1,
            Acks::All => -1,
        }
    }
}
