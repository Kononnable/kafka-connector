use crate::clients::producer::partitioner::Partitioner;

#[derive(Clone, Debug, Default)]
pub struct Crc32Partitioner {}

impl Partitioner for Crc32Partitioner {
    fn calculate_partition_index(&self, key: &[u8], partitions: usize) -> i32 {
        // TODO: Implement, test, check consistency with librdkafka, add more partitioner types
        0
    }
}
