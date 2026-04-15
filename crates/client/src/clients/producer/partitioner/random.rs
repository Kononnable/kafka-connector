use crate::clients::producer::partitioner::Partitioner;

#[derive(Clone, Debug, Default)]
pub struct RandomPartitioner {}
impl RandomPartitioner {
    pub fn new() -> RandomPartitioner {
        RandomPartitioner {}
    }
}

impl Partitioner for RandomPartitioner {
    fn calculate_partition_index(&self, _key: &[u8], partition_count: usize) -> i32 {
        (rand::random::<u32>() % partition_count as u32) as i32
    }
}
