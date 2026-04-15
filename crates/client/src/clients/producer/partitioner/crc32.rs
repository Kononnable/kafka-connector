use crate::clients::producer::partitioner::random::RandomPartitioner;
use crate::clients::producer::partitioner::{EmptyKeyPartitioningStrategy, Partitioner};
use crc::{CRC_32_ISO_HDLC, Crc};

static CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Clone, Debug, Default)]
pub struct Crc32Partitioner {
    random_partitioner: Option<RandomPartitioner>,
}
impl Crc32Partitioner {
    pub fn new(empty_key_strategy: EmptyKeyPartitioningStrategy) -> Crc32Partitioner {
        let random_partitioner = matches!(empty_key_strategy, EmptyKeyPartitioningStrategy::Random)
            .then(RandomPartitioner::new);
        Crc32Partitioner { random_partitioner }
    }

    pub fn hash(data: &[u8]) -> u32 {
        CRC32.checksum(data)
    }
}

impl Partitioner for Crc32Partitioner {
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
        let test_cases: [(&[u8], u32); _] = [
            // librdkafka UTs
            (&[], 0x0),
            (b"23456", 0xb1b451d7),
            (
                b"this is another string with more length to it perhaps",
                0xb0150df7,
            ),
            (b"hejsan", 0xd077037e),
        ];

        for (input, expected) in test_cases {
            assert_eq!(Crc32Partitioner::hash(input), expected,);
        }
    }
}
