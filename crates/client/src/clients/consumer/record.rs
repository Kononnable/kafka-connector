use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Record {
    pub timestamp: SystemTime,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub topic: String,
    pub headers: HashMap<String, Vec<u8>>,
    pub partition: i32,
    pub offset: i64,
}
