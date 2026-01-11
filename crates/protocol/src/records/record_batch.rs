use crate::records::record::Record;
use crate::{ApiVersion, FromBytes, ToBytes};
use bitflags::bitflags;
use bytes::BytesMut;
use std::mem;
use std::ops::Add;
use std::time::{Duration, SystemTime};

// TODO: remove pub from fields that are calculated, add Default to allow object creation
// batch_length, magic, crc, last_offset_delta, base_timestamp, max_timestamp
// offset_delta, timestamp_delta - absolute values in Record, will require changes to not use FromBytes(?)
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RecordBatch {
    pub base_offset: i64,
    /// Will be calculated automatically
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    /// Only magic byte = 2 is supported (Kafka >0.10)
    pub magic: i8,
    /// Will be calculated automatically
    pub crc: u32,
    pub attributes: i16,
    /// Will be calculated automatically
    pub last_offset_delta: i32,
    pub base_timestamp: SystemTime,
    /// Will be calculated automatically
    pub max_timestamp: SystemTime,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: Vec<Record>,
}

bitflags! {
    struct BatchAttributes: i16 {
    const GZIP = 0b00000001;
    const SNAPPY = 0b0000010;
    const LZ4 = 0b00000011;
    const ZSTD = 0b00000100;
    const TIMESTAMP_TYPE = 0b00001000;
    const IS_TRANSACTIONAL = 0b00010000;
    const IS_CONTROL_BATCH = 0b00100000;
    const HAS_DELETE_HORIZON_MS = 0b01000000;
    const IS_COMPRESSED = Self::GZIP.bits() | Self::SNAPPY.bits() | Self::LZ4.bits() | Self::ZSTD.bits();
    }
}
impl Default for RecordBatch {
    fn default() -> Self {
        Self {
            base_offset: 0,
            batch_length: 0,
            partition_leader_epoch: 0,
            magic: 2,
            crc: 0,
            attributes: 0,
            last_offset_delta: 0,
            base_timestamp: SystemTime::UNIX_EPOCH,
            max_timestamp: SystemTime::UNIX_EPOCH,
            producer_id: -1, // no idempotence
            producer_epoch: 0,
            base_sequence: 0,
            records: vec![],
        }
    }
}

// println!("{:#x?}", bytes.iter().as_slice());
impl RecordBatch {
    pub fn decode(bytes: &mut BytesMut) -> RecordBatch {
        let base_offset = FromBytes::deserialize(ApiVersion(0), bytes);
        let batch_length: i32 = FromBytes::deserialize(ApiVersion(0), bytes);
        let partition_leader_epoch = FromBytes::deserialize(ApiVersion(0), bytes);
        let magic: i8 = FromBytes::deserialize(ApiVersion(0), bytes);
        assert_eq!(magic, 2, "Kafka <0.11 not supported");
        let crc: u32 = FromBytes::deserialize(ApiVersion(0), bytes);
        let mut crc_buffer = bytes.split_to(
            batch_length as usize
                - mem::size_of::<i32>() // partition_leader_epoch
                - mem::size_of::<i8>() // magic
                - mem::size_of::<u32>(), // crc
        );
        let crc_calculated = crc32c::crc32c(&crc_buffer);
        assert_eq!(crc, crc_calculated); // TODO: what to do if crc does not match
        let bytes = &mut crc_buffer;
        let attributes = FromBytes::deserialize(ApiVersion(0), bytes);

        let last_offset_delta = FromBytes::deserialize(ApiVersion(0), bytes);
        let base_timestamp = SystemTime::UNIX_EPOCH
            .add(Duration::from_millis(
                i64::deserialize(ApiVersion(0), bytes) as u64,
            ));
        let max_timestamp = SystemTime::UNIX_EPOCH
            .add(Duration::from_millis(
                i64::deserialize(ApiVersion(0), bytes) as u64,
            ));
        let producer_id = FromBytes::deserialize(ApiVersion(0), bytes);
        let producer_epoch = FromBytes::deserialize(ApiVersion(0), bytes);
        let base_sequence = FromBytes::deserialize(ApiVersion(0), bytes);
        let records = FromBytes::deserialize(ApiVersion(0), bytes);

        RecordBatch {
            base_offset,
            batch_length,
            partition_leader_epoch,
            magic,
            crc,
            attributes,
            last_offset_delta,
            base_timestamp,
            max_timestamp,
            producer_id,
            producer_epoch,
            base_sequence,
            records,
        }
    }
    /// Encodes struct to kafka format. Some field values are calculated during encoding.
    pub fn encode(&mut self, bytes: &mut BytesMut) {
        assert!(!self.records.is_empty());
        assert_eq!(self.magic, 2);

        // Calculate values

        // last_offset_delta, max_timestamp, base_timestamp
        self.last_offset_delta = self
            .records
            .iter()
            .max_by_key(|x| x.offset_delta)
            .unwrap()
            .offset_delta
            .0;

        let min_timestamp = self
            .records
            .iter()
            .min_by_key(|x| x.timestamp_delta)
            .unwrap()
            .timestamp_delta
            .0;
        let max_timestamp = self
            .records
            .iter()
            .max_by_key(|x| x.timestamp_delta)
            .unwrap()
            .timestamp_delta
            .0;

        self.base_timestamp = SystemTime::UNIX_EPOCH + Duration::from_millis(min_timestamp as u64);
        self.records.iter_mut().for_each(|x| {
            x.timestamp_delta.0 -= min_timestamp;
        });
        self.max_timestamp = SystemTime::UNIX_EPOCH + Duration::from_millis(max_timestamp as u64);

        // Values calculated later on, reserving space
        let mut buf_batch_length = bytes.split_off(bytes.len());
        buf_batch_length.resize(4, 0);
        let mut buf_epoch_magic_crc = buf_batch_length.split_off(buf_batch_length.len());
        buf_batch_length.clear();
        // Batch length will be checked and serialized later

        buf_epoch_magic_crc.resize(4 + 1 + 4, 0);
        let mut crc_buffer = buf_epoch_magic_crc.split_off(buf_epoch_magic_crc.len());
        buf_epoch_magic_crc.clear();
        self.partition_leader_epoch
            .serialize(ApiVersion(0), &mut buf_epoch_magic_crc);
        self.magic
            .serialize(ApiVersion(0), &mut buf_epoch_magic_crc);
        //CRC will be calculated and serialized later

        let base_timestamp = self
            .base_timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        let max_timestamp = self
            .max_timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        self.attributes.serialize(ApiVersion(0), &mut crc_buffer);
        self.last_offset_delta
            .serialize(ApiVersion(0), &mut crc_buffer);
        base_timestamp.serialize(ApiVersion(0), &mut crc_buffer);
        max_timestamp.serialize(ApiVersion(0), &mut crc_buffer);
        self.producer_id.serialize(ApiVersion(0), &mut crc_buffer);
        self.producer_epoch
            .serialize(ApiVersion(0), &mut crc_buffer);
        self.base_sequence.serialize(ApiVersion(0), &mut crc_buffer);
        self.records.serialize(ApiVersion(0), &mut crc_buffer);

        self.crc = crc32c::crc32c(&crc_buffer);
        self.crc.serialize(ApiVersion(0), &mut buf_epoch_magic_crc);

        self.batch_length = (buf_epoch_magic_crc.len() + crc_buffer.len()) as i32;
        self.batch_length
            .serialize(ApiVersion(0), &mut buf_batch_length);

        // Write in the proper field order
        self.base_offset.serialize(ApiVersion(0), bytes);
        bytes.unsplit(buf_batch_length);
        bytes.unsplit(buf_epoch_magic_crc);
        bytes.unsplit(crc_buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::base_types::{VarInt, VarIntBytes, VarIntString, VarIntVec, VarLong};
    use crate::records::header::Header;
    use crate::records::record::Record;
    use bytes::BufMut;

    #[test]
    fn basic_decode_encode() {
        let packet_bytes: [u8; 96] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x54, 0x00, 0x00,
            0x00, 0x00, 0x02, 0xf0, 0x15, 0x91, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x9b, 0x76, 0xce, 0xb2, 0x5d, 0x00, 0x00, 0x01, 0x9b, 0x76, 0xce, 0xb2,
            0x5d, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0x00, 0x00, 0x00, 0x01, 0x44, 0x00, 0x00, 0x00, 0x12, 0x4e, 0x34, 0x52, 0x64,
            0x6d, 0x57, 0x42, 0x35, 0x30, 0x0a, 0x51, 0x66, 0x44, 0x33, 0x38, 0x02, 0x10, 0x61,
            0x70, 0x70, 0x2e, 0x6e, 0x61, 0x6d, 0x65, 0x08, 0x74, 0x65, 0x73, 0x74,
        ];
        let mut bytes = BytesMut::from(packet_bytes.as_slice());
        bytes.put_slice(&[0x00, 0x00]);
        let decoded = RecordBatch::decode(&mut bytes);
        let decoded_batch = RecordBatch {
            base_offset: 3,
            batch_length: 84,
            partition_leader_epoch: 0,
            magic: 2,
            crc: 0xf0159141,
            attributes: 0,
            last_offset_delta: 0,
            base_timestamp: SystemTime::UNIX_EPOCH.add(Duration::from_millis(1767224816221)), // 2025-12-31T23:46:56.221000000Z
            max_timestamp: SystemTime::UNIX_EPOCH.add(Duration::from_millis(1767224816221)), // 2025-12-31T23:46:56.221000000Z
            producer_id: -1,
            producer_epoch: -1,
            base_sequence: -1,
            records: vec![Record {
                attributes: 0,
                timestamp_delta: VarLong(0),
                offset_delta: VarInt(0),
                key: VarIntBytes(
                    "N4RdmWB50"
                        .as_bytes()
                        .iter()
                        .map(ToOwned::to_owned)
                        .collect(),
                ),
                value: VarIntBytes("QfD38".as_bytes().iter().map(ToOwned::to_owned).collect()),
                headers: VarIntVec(vec![Header {
                    header_key: VarIntString("app.name".to_owned()),
                    value: VarIntBytes("test".as_bytes().to_owned()),
                }]),
            }],
        };
        assert_eq!(decoded, decoded_batch);
        assert_eq!(bytes.len(), 2);

        let mut encoded = BytesMut::new();
        let mut batch_after_encoding = RecordBatch {
            batch_length: 0,
            magic: 2,
            crc: 0,
            last_offset_delta: 0,
            max_timestamp: SystemTime::UNIX_EPOCH,
            ..decoded_batch.clone()
        };
        batch_after_encoding.encode(&mut encoded);
        assert_eq!(decoded_batch, batch_after_encoding);
        let bytes = BytesMut::from(packet_bytes.as_slice());
        assert_eq!(bytes, encoded);
    }
}
