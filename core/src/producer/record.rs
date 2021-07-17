use std::collections::HashMap;
use typed_builder::TypedBuilder;

// TODO: Allow some simple types to be passed here directly - like str - utilize serializers from producer options
// TODO: Check if can be replaced by not owning counterparts(might be problems with static lifetime)
#[derive(Clone, TypedBuilder, Default, Debug)]
pub struct ProducerRecord {
    pub topic: String,
    pub payload: Vec<u8>,
    #[builder(default)]
    partition: Option<i32>,
    #[builder(default)]
    pub key: Vec<u8>,
    #[builder(default)]
    pub timestamp: Option<i64>,
    #[builder(default)]
    pub headers: Option<HashMap<String, Vec<u8>>>,
}

impl ProducerRecord {
    pub fn partition(&self) -> i32 {
        // TODO: Partitioner
        self.partition.unwrap_or_default()
    }
}
