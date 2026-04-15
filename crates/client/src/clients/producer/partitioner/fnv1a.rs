use crate::clients::producer::partitioner::random::RandomPartitioner;
use crate::clients::producer::partitioner::{EmptyKeyPartitioningStrategy, Partitioner};

#[derive(Clone, Debug, Default)]
pub struct Fnv1aPartitioner {
    random_partitioner: Option<RandomPartitioner>,
}
impl Fnv1aPartitioner {
    pub fn new(empty_key_strategy: EmptyKeyPartitioningStrategy) -> Fnv1aPartitioner {
        let random_partitioner = matches!(empty_key_strategy, EmptyKeyPartitioningStrategy::Random)
            .then(RandomPartitioner::new);
        Fnv1aPartitioner { random_partitioner }
    }

    pub fn hash(data: &[u8]) -> u32 {
        const PRIME: u32 = 0x01000193;
        const OFFSET: u32 = 0x811C9DC5;

        let mut h = OFFSET as i32;

        for &byte in data {
            h ^= byte as i32;
            h = h.wrapping_mul(PRIME as i32)
        }

        h.unsigned_abs()
    }
}

impl Partitioner for Fnv1aPartitioner {
    fn calculate_partition_index(&self, key: &[u8], partition_count: usize) -> i32 {
        if key.is_empty() {
            match &self.random_partitioner {
                None => 0,
                Some(partitioner) => partitioner.calculate_partition_index(key, partition_count),
            }
        } else {
            (Self::hash(key) % partition_count as u32) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        const SHORT_UNALIGNED: &[u8] = b"1234";
        const UNALIGNED: &[u8] = b"PreAmbleWillBeRemoved,ThePrePartThatIs";

        let test_cases: [(&[u8], u32); _] = [
            // librdkafka UTs
            (b"kafka", 0xd33c4e1),
            (b"giberish123456789", 0x77a58295),
            (SHORT_UNALIGNED, 0x23bdd03),
            (&SHORT_UNALIGNED[1..], 0x2dea3cd2),
            (&SHORT_UNALIGNED[2..], 0x740fa83e),
            (&SHORT_UNALIGNED[3..], 0x310ca263),
            (UNALIGNED, 0x65cbd69c),
            (&UNALIGNED[1..], 0x6e49c79a),
            (&UNALIGNED[2..], 0x69eed356),
            (&UNALIGNED[3..], 0x6abcc023),
            (&[], 0x7ee3623b),
            (b"23456", 0x27e6f469),
            (
                b"this is another string with more length to it perhaps",
                0x155e3e5f,
            ),
            (b"hejsan", 0x17b1e27a),
        ];

        for (input, expected) in test_cases {
            assert_eq!(Fnv1aPartitioner::hash(input), expected);
        }
    }
}
