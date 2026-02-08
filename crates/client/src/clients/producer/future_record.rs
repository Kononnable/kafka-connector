use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Default)]
pub struct FutureRecord {
    /// If empty, it will be set automatically when message is sent
    pub timestamp: Option<SystemTime>,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub topic: String,
    /// Set only if you want to force a specific partition, skipping partitioner mechanism
    pub partition: Option<i16>,
    pub headers: HashMap<String, Vec<u8>>,
}

impl FutureRecord {
    pub fn new<T, K, V>(topic: T, key: K, value: V) -> FutureRecord
    where
        T: Into<String>,
        K: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        FutureRecord {
            topic: topic.into(),
            key: key.into(),
            value: value.into(),
            ..FutureRecord::default()
        }
    }
    pub fn set_timestamp<T>(&mut self, ts: T) -> &mut Self
    where
        T: Into<SystemTime>,
    {
        self.timestamp.replace(ts.into());
        self
    }
    pub fn add_header<K>(&mut self, key: String, value: Vec<u8>) -> &mut Self
    where
        K: Into<Vec<u8>>,
    {
        self.headers.insert(key, value);
        self
    }
}
