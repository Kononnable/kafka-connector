use std::fmt::Debug;

pub mod crc32;
pub mod fnv1a;
pub mod murmur2;
pub mod random;

pub enum EmptyKeyPartitioningStrategy {
    /// Messages with empty key will be distributed randomly among partitions.
    /// No ordering guarantees will be preserved for such messages.
    Random,
    /// All messages with empty key will be delivered to the same partition.
    /// Standard ordering guarantees will apply.
    /// Will cause uneven partition load if there are many messages without keys.
    SamePartition,
}

pub trait Partitioner: Clone + Debug + Send + Sync + 'static {
    /// `key` - kafka message key
    ///
    /// `partition_count` -  number of partitions in the topic
    ///
    /// `return value` - partition index the message is assigned to (zero based)
    ///
    fn calculate_partition_index(&self, key: &[u8], partition_count: usize) -> i32;
}
