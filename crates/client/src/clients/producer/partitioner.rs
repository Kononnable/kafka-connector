use std::fmt::Debug;

pub mod crc32;

pub trait Partitioner: Clone + Debug + Send + 'static {
    /// `key` - kafka message key
    ///
    /// `partitions` -  number of partitions in the topic
    ///
    /// `return value` - partition index the message is assigned to (zero based)
    ///
    fn calculate_partition_index(&self, key: &[u8], partitions: usize) -> i32;
}
