use derivative::Derivative;
use std::collections::HashSet;
use std::time::Duration;

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct KafkaConsumerOptions {
    /// Topics to subscribe to
    pub topics: HashSet<String>,

    /// Maximum time for fetch response to be delayed to collect more messages.
    #[derivative(Default(value = "Duration::from_millis(500)"))]
    pub max_wait: Duration,

    /// Minimum amount of data fetch response should return. If not enough data is collected
    /// within `max_wait` response will less amount of data will be returned.
    #[derivative(Default(value = "1"))]
    pub min_bytes: i32,

    /// Maximum amount of data fetch response should return.
    #[derivative(Default(value = "50 * 1024 * 1024"))]
    pub max_bytes: i32,

    /// Maximum amount of data for single partition fetch response should return.
    ///
    /// Should not be higher than `max_bytes`.
    /// This limit will not be respected if message batch is larger than the limit.
    #[derivative(Default(value = "1 * 1024 * 1024"))]
    pub max_bytes_per_partition: i32,
}
