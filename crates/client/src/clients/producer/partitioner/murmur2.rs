use crate::clients::producer::partitioner::random::RandomPartitioner;
use crate::clients::producer::partitioner::{EmptyKeyPartitioningStrategy, Partitioner};

#[derive(Clone, Debug, Default)]
pub struct Murmur2Partitioner {
    // used only for null keys
    random_partitioner: Option<RandomPartitioner>,
}
impl Murmur2Partitioner {
    pub fn new(empty_key_strategy: EmptyKeyPartitioningStrategy) -> Murmur2Partitioner {
        let random_partitioner = matches!(empty_key_strategy, EmptyKeyPartitioningStrategy::Random)
            .then(RandomPartitioner::new);
        Murmur2Partitioner { random_partitioner }
    }

    pub fn hash(data: &[u8]) -> u32 {
        const SEED: u32 = 0x9747b28c;
        const M: u32 = 0x5bd1e995;
        const R: u32 = 24;

        let mut h = SEED ^ (data.len() as u32);

        let mut chunks = data.chunks_exact(4);

        for chunk in &mut chunks {
            let mut k = u32::from_le_bytes(chunk.try_into().unwrap());

            k = k.wrapping_mul(M);
            k ^= k >> R;
            k = k.wrapping_mul(M);

            h = h.wrapping_mul(M);
            h ^= k;
        }

        match chunks.remainder() {
            [a, b, c] => {
                h ^= (*c as u32) << 16;
                h ^= (*b as u32) << 8;
                h ^= *a as u32;
                h = h.wrapping_mul(M);
            }
            [a, b] => {
                h ^= (*b as u32) << 8;
                h ^= *a as u32;
                h = h.wrapping_mul(M);
            }
            [a] => {
                h ^= *a as u32;
                h = h.wrapping_mul(M);
            }
            [] => {}
            _ => unreachable!(),
        }

        h ^= h >> 13;
        h = h.wrapping_mul(M);
        h ^= h >> 15;

        h
    }
}

impl Partitioner for Murmur2Partitioner {
    fn calculate_partition_index(&self, key: &[u8], partition_count: usize) -> i32 {
        if key.is_empty() {
            match &self.random_partitioner {
                None => 0,
                Some(partitioner) => partitioner.calculate_partition_index(key, partition_count),
            }
        } else {
            ((Self::hash(key) & 0x7fff_ffffu32) % partition_count as u32) as i32
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
            // java client UTs
            (b"21", -973932308_i32 as u32),
            (b"foobar", -790332482_i32 as u32),
            (b"a-little-bit-long-string", -985981536_i32 as u32),
            (b"a-little-bit-longer-string", -1486304829_i32 as u32),
            (
                b"lkjh234lh9fiuh90y23oiuhsafujhadof229phr9h19h89h8",
                -58897971_i32 as u32,
            ),
            (b"abc", 479470107_i32 as u32),
            // librdkafka UTs
            (b"kafka", 0xd067cf64_u32),
            (b"giberish123456789", 0x8f552b0c_u32),
            (SHORT_UNALIGNED, 0x9fc97b14_u32),
            (&SHORT_UNALIGNED[1..], 0xe7c009ca_u32),
            (&SHORT_UNALIGNED[2..], 0x873930da_u32),
            (&SHORT_UNALIGNED[3..], 0x5a4b5ca1_u32),
            (UNALIGNED, 0x78424f1c_u32),
            (&UNALIGNED[1..], 0x4a62b377_u32),
            (&UNALIGNED[2..], 0xe0e4e09e_u32),
            (&UNALIGNED[3..], 0x62b8b43f_u32),
            (&[], 0x106e08d9_u32),
        ];

        for (input, expected) in test_cases {
            assert_eq!(Murmur2Partitioner::hash(input), expected);
        }
    }

    #[test]
    fn calculate_partition_index() {
        const PARTITION_COUNT: i32 = 7;
        let test_cases: [(&[u8], _); _] = [
            // librdkafka UTs
            (b"", 0), // EmptyKeyPartitioningStrategy::SamePartition
            (b"23456", 0x058d780f % PARTITION_COUNT),
            (
                b"this is another string with more length to it perhaps",
                0x4f7703da % PARTITION_COUNT,
            ),
            (b"hejsan", 0x5ec19395 % PARTITION_COUNT),
        ];

        let partitioner = Murmur2Partitioner::new(EmptyKeyPartitioningStrategy::SamePartition);
        for (input, expected) in test_cases {
            assert_eq!(
                partitioner.calculate_partition_index(input, PARTITION_COUNT as usize),
                expected
            );
        }
    }
}
