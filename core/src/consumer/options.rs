use typed_builder::TypedBuilder;

#[derive(Clone, TypedBuilder, Default, Debug)]
pub struct ConsumerOptions {
    // allow.auto.create.topics
    // auto.commit.interval.ms
    // auto.offset.reset
    // check.crcs
    #[builder(default = {"".to_owned()})]
    pub client_rack: String,
    // enable.auto.commit
    // exclude.internal.topics
    #[builder(default = {52_428_800})]
    pub fetch_max_bytes: i32,
    #[builder(default = {500})]
    pub fetch_max_wait_ms: i32,
    #[builder(default = {1})]
    pub fetch_min_bytes: i32,
    pub group_id: String,
    // TODO: Allow some regexp, not only topic names
    pub topics: Vec<String>,
    // group.instance.id
    #[builder(default = {2_000})]
    pub heartbeat_interval_ms: u64,
    #[builder(default = {1_048_576})]
    pub max_partition_fetch_bytes: i32,
    // max.poll.interval.ms
    // max.poll.records
    // session.timeout.ms
    // default.api.timeout.ms
    // isolation.level // transactions
}
