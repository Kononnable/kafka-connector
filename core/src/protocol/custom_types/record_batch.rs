use bytes::{Bytes, BytesMut};
use std::fmt::Debug;

use crate::protocol::{from_bytes::FromBytes, to_bytes::ToBytes};
use kafka_connector_derive::FromBytes;
use kafka_connector_derive::ToBytes;

use super::{
    zig_zag_string::ZigZagString, zig_zag_varint64::ZigZagVarInt64, zig_zag_vec::ZigZagVec,
};

#[derive(Debug, Default, FromBytes)]
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
#[derive(Debug, Default, FromBytes)]
pub struct Record {
    pub length: ZigZagVarInt64,
    pub attributes: i8,
    pub timestamp_delta: ZigZagVarInt64,
    pub offset_delta: ZigZagVarInt64,
    pub key: ZigZagVec<u8>,
    pub value: ZigZagVec<u8>,
    pub headers: ZigZagVec<Header>,
}

#[derive(Debug, Default, ToBytes, FromBytes)]
pub struct Header {
    pub key: ZigZagString,
    pub value: ZigZagVec<u8>,
}

impl ToBytes for RecordBatch {
    fn serialize(&self, buf: &mut BytesMut) {
        // MessageSet v2 header(?)
        let message_set_v2_header_size: i32 = 8 + 4 + 4 + 1 + 4 + 2 + 4 + 8 + 8 + 8 + 2 + 4 + 4;
        message_set_v2_header_size.serialize(buf);

        self.base_offset.serialize(buf);
        self.batch_length.serialize(buf); // set later(?)
        self.partition_leader_epoch.serialize(buf);
        self.magic.serialize(buf);
        self.crc.serialize(buf); // set later(?)
        self.attributes.serialize(buf); // set later(?)
        self.last_offset_delta.serialize(buf); // set later(?)
        self.first_timestamp.serialize(buf); // set later(?)
        self.max_timestamp.serialize(buf); // set later
        self.producer_id.serialize(buf);
        self.producer_epoch.serialize(buf);
        self.base_sequence.serialize(buf); // set later
        self.records.serialize(buf); // set later

        todo!()
    }
}

impl ToBytes for Record {
    fn serialize(&self, buf: &mut BytesMut) {
        self.length.serialize(buf); // set later(?)
        self.attributes.serialize(buf); // set later(?)
        self.timestamp_delta.serialize(buf); // set later(?)
        self.offset_delta.serialize(buf); // set later(?)
        self.key.serialize(buf);
        self.value.serialize(buf);
        self.headers.serialize(buf); // TODO: set later(?) - no len, len as zigzag i32
        todo!()
    }
}
