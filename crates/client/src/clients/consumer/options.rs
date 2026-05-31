use crate::clients::consumer::consumer_loop::assignment_strategy::ConsumerAssignmentStrategy;
use crate::clients::consumer::consumer_loop::assignment_strategy::round_robin::RoundRobin;
use crate::protocol_consts::ListOffsetsTimestampType;
use derivative::Derivative;
use std::collections::HashSet;
use std::ops::Sub;
use std::sync::Arc;
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

    /// Consumer group identifier.
    ///
    /// If set to `None` consumer will act without a group, without commiting any processed offsets and ability to
    /// resume work after restart.
    pub group_id: Option<String>,

    /// Timeout used to detect consumer failures.
    ///
    /// If consumer does not send a heartbeat within the timeout broker will consider consumer dead and initiate
    /// a consumer group rebalance.
    #[derivative(Default(value = "Duration::from_secs(45)"))]
    pub session_timeout: Duration,

    // TODO: keep separate or implement like in java client with max.poll.interval.ms; docs
    #[derivative(Default(value = "Duration::from_secs(300)"))]
    pub rebalance_timeout: Duration,

    // TODO: docs, make sure it's not empty if group is set(?) default values
    // note on arc - copying consumer options side effects (internal mutability of implementation + reuse same instance through arc)
    #[derivative(Default(value = "vec![Arc::new(RoundRobin{})]"))]
    // TODO: change to dyn trait when trait interface is defined
    // pub assignment_strategies: Vec<Arc<dyn ConsumerAssignmentStrategy>>,
    pub assignment_strategies: Vec<Arc<dyn ConsumerAssignmentStrategy>>,
}

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
                .sub(duration)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        }
    }
}
