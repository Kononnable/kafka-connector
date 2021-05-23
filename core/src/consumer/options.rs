use typed_builder::TypedBuilder;

#[derive(Clone, TypedBuilder, Default, Debug)]
pub struct ConsumerOptions {
    // allow.auto.create.topics
    // auto.commit.interval.ms
    // auto.offset.reset
    // check.crcs
    // client.rack
    // enable.auto.commit
    // exclude.internal.topics
    // fetch.max.bytes
    // fetch.max.wait.ms
    // fetch.min.bytes
    pub group_id: String,
    // TODO: Allow some regexp, not only topic names
    pub topics: Vec<String>,
    // group.instance.id
    // heartbeat.interval.ms
    // max.partition.fetch.bytes
    // max.poll.interval.ms
    // max.poll.records
    // session.timeout.ms
    // default.api.timeout.ms
    // isolation.level // transactions
}
