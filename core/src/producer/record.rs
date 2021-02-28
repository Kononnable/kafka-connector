use std::collections::HashMap;

// TODO: Allow some simple types to be passed here directly - like str - utilize serializers from producer options
pub struct ProducerRecord<'a> {
    pub topic: &'a str,
    pub payload: &'a [u8],
    pub partition: Option<i32>,
    pub key: &'a [u8],
    pub timestamp: Option<i64>,
    pub headers: Option<&'a HashMap<String, Vec<u8>>>,
}
