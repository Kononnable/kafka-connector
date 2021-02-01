use bytes::{Bytes, BytesMut};

use kafka_connector_derive::FromBytes;

use crate::{
    custom_types::{zig_zag_varint64::ZigZagVarInt64, zig_zag_vec::ZigZagVec},
    from_bytes::FromBytes,
    to_bytes::ToBytes,
};

use super::header::Header;

#[derive(Debug, Default, FromBytes, Clone)]
pub struct Record {
    pub length: ZigZagVarInt64,
    pub attributes: i8,
    pub timestamp_delta: ZigZagVarInt64,
    pub offset_delta: ZigZagVarInt64,
    pub key: ZigZagVec<u8>,
    pub value: ZigZagVec<u8>,
    pub headers: ZigZagVec<Header>,
}

impl ToBytes for Record {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        self.length.serialize(buf, is_flexible_version); // set later(?)
        self.attributes.serialize(buf, is_flexible_version); // set later(?)
        self.timestamp_delta.serialize(buf, is_flexible_version); // set later(?)
        self.offset_delta.serialize(buf, is_flexible_version); // set later(?)
        self.key.serialize(buf, is_flexible_version);
        self.value.serialize(buf, is_flexible_version);
        self.headers.serialize(buf, is_flexible_version); // TODO: set later(?) - no len, len as zigzag i32
        todo!()
    }
}
