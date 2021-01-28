use bytes::{ Bytes, BytesMut};

use crate::protocol::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::unsigned_varint64::{ UnsignedVarInt64};

pub fn decode_zig_zag_64(n: u64) -> i64 {
    ((n >> 1) as i64) ^ (-((n & 1) as i64))
}

pub fn encode_zig_zag_64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

#[derive(Debug)]
pub struct ZigZagVarInt64 {
    pub value: i64,
}
impl ZigZagVarInt64 {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}
impl FromBytes for ZigZagVarInt64 {
    fn deserialize(buf: &mut Bytes) -> Self {
        let varint = UnsignedVarInt64::deserialize(buf);
        let value = decode_zig_zag_64(varint.value);
        ZigZagVarInt64 { value }
    }
}

impl ToBytes for ZigZagVarInt64 {
    fn serialize(&self, buf: &mut BytesMut) {
        let zigzag = encode_zig_zag_64(self.value);
        let varint = UnsignedVarInt64::new(zigzag);
        varint.serialize(buf);
    }
}
impl Default for ZigZagVarInt64 {
    fn default() -> Self {
        ZigZagVarInt64 { value: 0 }
    }
}

impl From<i64> for ZigZagVarInt64 {
    fn from(from: i64) -> Self {
        ZigZagVarInt64 { value: from }
    }
}

impl From<ZigZagVarInt64> for i64 {
    fn from(from: ZigZagVarInt64) -> Self {
        from.value
    }
}
