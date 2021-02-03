use bitflags::bitflags;
use bytes::{BufMut, Bytes, BytesMut};
use log::trace;
use std::fmt::Debug;

use crate::{
    custom_types::{
        unsigned_varint32::UnsignedVarInt32, zig_zag_varint32::ZigZagVarInt32,
        zig_zag_varint64::ZigZagVarInt64, zig_zag_vec::ZigZagVec,
    },
    from_bytes::FromBytes,
    to_bytes::ToBytes,
};

use super::{header::Header, record::Record};

#[derive(Debug, Default, Clone)]
pub struct RecordBatch {
    pub partition_leader_epoch: i32,
    pub attributes: i16,
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
impl RecordBatch {
    fn serialize_record(
        record: &Record,
        buf: &mut BytesMut,
        is_flexible_version: bool,
        base_timestamp: i64,
        base_offset: i64,
    ) {
        let mut buffer = BytesMut::with_capacity(4096); // TODO: Change size(?)

        let timestamp_delta = ZigZagVarInt64::new(record.timestamp - base_timestamp);
        let offset_delta = ZigZagVarInt64::new(record.offset - base_offset);
        trace!("Serializing record");

        record
            .attributes
            .serialize(&mut buffer, is_flexible_version);
        log::trace!("Serialized attributes Bytes: \n{:03?}", buffer.to_vec());
        timestamp_delta.serialize(&mut buffer, is_flexible_version);
        log::trace!(
            "Serialized timestamp_delta Bytes: \n{:03?}",
            buffer.to_vec()
        );
        offset_delta.serialize(&mut buffer, is_flexible_version);
        log::trace!("Serialized offset_delta Bytes: \n{:03?}", buffer.to_vec());
        record.key.serialize(&mut buffer, is_flexible_version);
        log::trace!("Serialized key Bytes: \n{:03?}", buffer.to_vec());
        record.value.serialize(&mut buffer, is_flexible_version);
        log::trace!("Serialized value Bytes: \n{:03?}", buffer.to_vec());
        record.headers.serialize(&mut buffer, is_flexible_version);
        log::trace!("Serialized headers Bytes: \n{:03?}", buffer.to_vec());

        let length = ZigZagVarInt64::new(buffer.len() as i64);
        length.serialize(buf, is_flexible_version);
        buf.extend(buffer);
    }
    fn deserialize_records(
        buf: &mut Bytes,
        is_flexible_version: bool,
        base_timestamp: i64,
        base_offset: i64,
    ) -> Vec<Record> {
        let cap: i32 = match is_flexible_version {
            true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i32 - 1,
            false => FromBytes::deserialize(buf, is_flexible_version),
        };
        if cap == -1 || cap == 0 {
            return vec![];
        }
        let mut ret = Vec::with_capacity(cap as usize);
        trace!("Deserialized cap \n{:#?}", cap);
        for _i in 0..cap {
            let len = ZigZagVarInt32::deserialize(buf, is_flexible_version);
            trace!("Deserialized len \n{:#?}", len);
            let attributes: i8 = FromBytes::deserialize(buf, is_flexible_version);
            trace!("Deserialized attributes \n{:#?}", attributes);
            let timestamp_delta: ZigZagVarInt64 = FromBytes::deserialize(buf, is_flexible_version);
            trace!("Deserialized timestamp_delta \n{:#?}", timestamp_delta);
            let offset_delta: ZigZagVarInt64 = FromBytes::deserialize(buf, is_flexible_version);
            trace!("Deserialized offset_delta \n{:#?}", offset_delta);
            let key: ZigZagVec<u8> = FromBytes::deserialize(buf, is_flexible_version);
            trace!("Deserialized key \n{:#?}", key);
            let value: ZigZagVec<u8> = FromBytes::deserialize(buf, is_flexible_version);
            trace!("Deserialized value \n{:#?}", value);
            let headers: ZigZagVec<Header> = FromBytes::deserialize(buf, is_flexible_version);
            trace!("Deserialized headers \n{:#?}", headers);

            let element = Record {
                attributes,
                timestamp: base_timestamp + timestamp_delta.value,
                offset: base_offset + offset_delta.value,
                key,
                value,
                headers,
            };
            ret.push(element);
            trace!("Bytes left: {:03?}", buf.to_vec());
        }
        ret
    }
}

impl ToBytes for RecordBatch {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        let base_offset: i64 = self.records.iter().min_by_key(|x| x.offset).unwrap().offset;
        let last_offset_delta: i32 =
            (self.records.iter().max_by_key(|x| x.offset).unwrap().offset - base_offset) as i32;
        let first_timestamp: i64 = self
            .records
            .iter()
            .min_by_key(|x| x.timestamp)
            .unwrap()
            .timestamp;
        let max_timestamp: i64 = self
            .records
            .iter()
            .max_by_key(|x| x.timestamp)
            .unwrap()
            .timestamp;

        let mut crc_buffer = BytesMut::with_capacity(4096); // TODO: Change size(?)

        trace!("Serializing recordbatch");
        self.attributes
            .serialize(&mut crc_buffer, is_flexible_version);
        trace!("Serialized attributes \n{:03?}", crc_buffer.to_vec());
        last_offset_delta.serialize(&mut crc_buffer, is_flexible_version);
        trace!("Serialized last_offset_delta \n{:03?}", crc_buffer.to_vec());
        first_timestamp.serialize(&mut crc_buffer, is_flexible_version);
        trace!("Serialized first_timestamp \n{:03?}", crc_buffer.to_vec());
        max_timestamp.serialize(&mut crc_buffer, is_flexible_version);
        trace!("Serialized max_timestamp \n{:03?}", crc_buffer.to_vec());
        self.producer_id
            .serialize(&mut crc_buffer, is_flexible_version);
        trace!("Serialized producer_id \n{:03?}", crc_buffer.to_vec());
        self.producer_epoch
            .serialize(&mut crc_buffer, is_flexible_version);
        trace!("Serialized producer_epoch \n{:03?}", crc_buffer.to_vec());
        self.base_sequence
            .serialize(&mut crc_buffer, is_flexible_version);
        trace!("Serialized base_sequence \n{:03?}", crc_buffer.to_vec());

        match is_flexible_version {
            true => {
                UnsignedVarInt32::new(self.records.len() as u32 + 1)
                    .serialize(&mut crc_buffer, is_flexible_version);
            }
            false => {
                crc_buffer.put_i32(self.records.len() as i32);
            }
        }
        for record in self.records.iter() {
            RecordBatch::serialize_record(
                &record,
                &mut crc_buffer,
                is_flexible_version,
                first_timestamp,
                base_offset,
            );
        }

        let mut batch_buffer = BytesMut::with_capacity(4096); // TODO: Change size(?)
        self.partition_leader_epoch
            .serialize(&mut batch_buffer, is_flexible_version);
        let magic_byte = 2_i8;
        magic_byte.serialize(&mut batch_buffer, is_flexible_version);

        let crc = crc32c::crc32c(&crc_buffer);
        crc.serialize(&mut batch_buffer, is_flexible_version);
        batch_buffer.extend(crc_buffer);

        base_offset.serialize(buf, is_flexible_version);
        trace!("Serialized base_offset \n{:03?}", buf.to_vec());
        let batch_length: i32 = batch_buffer.len() as i32;
        batch_length.serialize(buf, is_flexible_version);
        trace!("Serialized batch_length \n{:03?}", buf.to_vec());
        buf.extend(batch_buffer);
    }
}

impl FromBytes for RecordBatch {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let base_offset: i64 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized base_offset \n{:#?}", base_offset);
        let length: i32 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized length \n{:#?}", length);
        let partition_leader_epoch: i32 = FromBytes::deserialize(buf, is_flexible_version);
        trace!(
            "Deserialized partition_leader_epoch \n{:#?}",
            partition_leader_epoch
        );
        let magic_byte: i8 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized magic_byte \n{:#?}", magic_byte);
        let crc: u32 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized crc \n{:#?}", crc);
        let attributes: i16 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized attributes \n{:#?}", attributes);
        let last_offset_delta: i32 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized last_offset_delta \n{:#?}", last_offset_delta);
        let base_timestamp: i64 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized base_timestamp \n{:#?}", base_timestamp);
        let max_timestamp: i64 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized max_timestamp \n{:#?}", max_timestamp);
        let pid: i64 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized pid \n{:#?}", pid);
        let producer_epoch: i16 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized producer_epoch \n{:#?}", producer_epoch);
        let base_sequence: i32 = FromBytes::deserialize(buf, is_flexible_version);
        trace!("Deserialized base_sequence \n{:#?}", base_sequence);
        trace!("Bytes left: {:03?}", buf.to_vec());
        let records: Vec<Record> =
            RecordBatch::deserialize_records(buf, is_flexible_version, base_timestamp, base_offset);

        let attributes_struct = BatchAttributes::from_bits_truncate(attributes);
        if attributes_struct.contains(BatchAttributes::IS_COMPRESSED) {
            log::trace!("IS_COMPRESSED");
        }
        if attributes_struct.contains(BatchAttributes::TIMESTAMP_TYP) {
            log::trace!("TIMESTAMP_TYP");
        }
        if attributes_struct.contains(BatchAttributes::IS_TRANSACTIONAL) {
            log::trace!("IS_TRANSACTIONAL");
        }
        if attributes_struct.contains(BatchAttributes::IS_CONTROL_BATCH) {
            log::trace!("IS_CONTROL_BATCH");
        }

        RecordBatch {
            base_sequence,
            attributes,
            partition_leader_epoch,
            producer_epoch,
            producer_id: pid,
            records,
        }
    }
}
