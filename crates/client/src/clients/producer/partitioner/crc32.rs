use crate::clients::producer::partitioner::Partitioner;

#[derive(Clone, Debug, Default)]
pub struct Crc32Partitioner {}

impl Partitioner for Crc32Partitioner {
    fn calculate_partition_index(&self, _key: &[u8], _partitions: usize) -> i32 {
        // TODO: Implement, test, check consistency with librdkafka, add more partitioner types
        0
    }
}
