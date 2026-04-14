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
    pub timeout: Option<Duration>,

    /// Partitioner that will be used for assigning messages to specific partitions
    pub partitioner: P,

    /// Amount of time to delay sending a message to a broker.
    ///
    /// Increasing this value can increase system throughput and its latency by lowering kafka protocol overhead.
    #[derivative(Default(value = "Duration::from_millis(5)"))]
    pub linger: Duration,

    /// Maximum size of record batch that will be sent to kafka broker.
    ///
    /// If a single record is larger than this value, it will still be sent as a single record in a batch.
    ///
    /// This value takes into account only user records, before compression, with partial protocol overhead.
    #[derivative(Default(value = "16 * 1024"))]
    pub batch_size_bytes: u32,

    /// Maximum number of unacknowledged produce requests for each broker
    #[derivative(Default(value = "5"))]
    pub max_requests_in_flight: u8,
}

impl KafkaProducerOptions<Crc32Partitioner> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Acks {
    /// No broker acknowledge messages
    NoAck,
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
