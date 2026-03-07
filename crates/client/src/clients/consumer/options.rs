use crate::protocol_consts::ListOffsetsTimestampType;
use derivative::Derivative;
use std::collections::HashSet;
use std::ops::Add;
use std::time::{Duration, SystemTime};

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
    /// This limit will not be respected if a single message batch is larger than the limit.
    #[derivative(Default(value = "1024 * 1024"))]
    pub max_bytes_per_partition: i32,

    /// Defines which offset to start consuming from if there is no previous offset stored, or if it is unavailable.
    #[derivative(Default(value = "OffsetReset::Latest"))]
    pub offset_reset: OffsetReset,
}

// TODO: E2E tests
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OffsetReset {
    /// Start consuming from the oldest available message
    Earliest,

    /// Consume only new messages
    Latest,

    /// Catch up messages from last x seconds
    FromNow(Duration),
}

impl From<OffsetReset> for i64 {
    fn from(value: OffsetReset) -> Self {
        match value {
            OffsetReset::Earliest => ListOffsetsTimestampType::Earliest.into(),
            OffsetReset::Latest => ListOffsetsTimestampType::Latest.into(),
            OffsetReset::FromNow(duration) => SystemTime::now()
                .add(duration)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        }
    }
}
