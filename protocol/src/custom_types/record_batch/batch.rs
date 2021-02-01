use bitflags::bitflags;
use bytes::{Bytes, BytesMut};
use std::fmt::Debug;

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::record::Record;

#[derive(Debug, Default, Clone)]
pub struct RecordBatch {
    pub base_offset: i64,
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    pub magic: i8,
    pub crc: i32,
    pub attributes: i16,
    pub last_offset_delta: i32,
    pub first_timestamp: i64,
    pub max_timestamp: i64,
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
        const TIMESTAMP_TYP = 0b00001000;
        const IS_TRANSACTIONAL = 0b00010000;
        const IS_CONTROL_BATCH = 0b00100000;

        const IS_COMPRESSED = Self::GZIP.bits | Self::SNAPPY.bits | Self::LZ4.bits | Self::ZSTD.bits;
    }
}

impl ToBytes for RecordBatch {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        // MessageSet v2 header(?)
        let message_set_v2_header_size: i32 = 8 + 4 + 4 + 1 + 4 + 2 + 4 + 8 + 8 + 8 + 2 + 4 + 4;
        message_set_v2_header_size.serialize(buf, is_flexible_version);

        self.base_offset.serialize(buf, is_flexible_version);
        self.batch_length.serialize(buf, is_flexible_version); // set later(?)
        self.partition_leader_epoch
            .serialize(buf, is_flexible_version);
        self.magic.serialize(buf, is_flexible_version);
        self.crc.serialize(buf, is_flexible_version); // set later(?)
        self.attributes.serialize(buf, is_flexible_version); // set later(?)
        self.last_offset_delta.serialize(buf, is_flexible_version); // set later(?)
        self.first_timestamp.serialize(buf, is_flexible_version); // set later(?)
        self.max_timestamp.serialize(buf, is_flexible_version); // set later
        self.producer_id.serialize(buf, is_flexible_version);
        self.producer_epoch.serialize(buf, is_flexible_version);
        self.base_sequence.serialize(buf, is_flexible_version); // set later
        self.records.serialize(buf, is_flexible_version); // set later

        todo!()
    }
}

impl FromBytes for RecordBatch {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let base_offset: i64 = FromBytes::deserialize(buf, is_flexible_version);
        println!("base_offset {:?}", base_offset);
        let length: i32 = FromBytes::deserialize(buf, is_flexible_version);
        println!("length {:?}", length);
        let partition_leader_epoch: i32 = FromBytes::deserialize(buf, is_flexible_version);
        println!("partition_leader_epoch {:?}", partition_leader_epoch);
        let magic: i8 = FromBytes::deserialize(buf, is_flexible_version);
        println!("magic {:?}", magic);
        let crc: i32 = FromBytes::deserialize(buf, is_flexible_version);
        println!("crc {:?}", crc);
        let attributes: i16 = FromBytes::deserialize(buf, is_flexible_version);
        println!("attributes {:?}", attributes);
        let last_offset_delta: i32 = FromBytes::deserialize(buf, is_flexible_version);
        println!("last_offset_delta {:?}", last_offset_delta);
        let last_offset = base_offset + last_offset_delta as i64;
        println!("last_offset {:?}", last_offset);
        let base_timestamp: i64 = FromBytes::deserialize(buf, is_flexible_version);
        println!("base_timestamp {:?}", base_timestamp);
        let max_timestamp: i64 = FromBytes::deserialize(buf, is_flexible_version);
        println!("max_timestamp {:?}", max_timestamp);
        let pid: i64 = FromBytes::deserialize(buf, is_flexible_version);
        println!("pid {:?}", pid);
        let producer_epoch: i16 = FromBytes::deserialize(buf, is_flexible_version);
        println!("producer_epoch {:?}", producer_epoch);
        let base_sequence: i32 = FromBytes::deserialize(buf, is_flexible_version);
        println!("base_sequence {:?}", base_sequence);
        let records: Vec<Record> = FromBytes::deserialize(buf, is_flexible_version);

        let attributes_struct = BatchAttributes::from_bits_truncate(attributes);
        if attributes_struct.contains(BatchAttributes::IS_COMPRESSED) {
            println!("IS_COMPRESSED");
        }
        if attributes_struct.contains(BatchAttributes::TIMESTAMP_TYP) {
            println!("TIMESTAMP_TYP");
        }
        if attributes_struct.contains(BatchAttributes::IS_TRANSACTIONAL) {
            println!("IS_TRANSACTIONAL");
        }
        if attributes_struct.contains(BatchAttributes::IS_CONTROL_BATCH) {
            println!("IS_CONTROL_BATCH");
        }

        let res = RecordBatch {
            base_offset,
            base_sequence,
            attributes,
            batch_length: length,
            crc,
            first_timestamp: base_timestamp,
            last_offset_delta,
            magic,
            max_timestamp,
            partition_leader_epoch,
            producer_epoch,
            producer_id: pid,
            records,
        };
        println!("{:#?}", res);
        res
        // let len: i32 = match is_flexible_version {
        //     true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i32 - 1,
        //     false => FromBytes::deserialize(buf, is_flexible_version),
        // };
        // if len == -1 || len == 0 {
        //     return vec![];
        // }
        // buf.split_to(len as usize).into_iter().collect()
    }
}
