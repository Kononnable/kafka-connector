use bytes::{Bytes, BytesMut};

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::unsigned_varint32::UnsignedVarInt32;

pub fn decode_zig_zag_32(n: u32) -> i32 {
    ((n >> 1) as i32) ^ (-((n & 1) as i32))
}

pub fn encode_zig_zag_32(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}

#[derive(Debug, Clone)]
pub struct ZigZagVarInt32 {
    pub value: i32,
}
impl ZigZagVarInt32 {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
impl FromBytes for ZigZagVarInt32 {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        let varint = UnsignedVarInt32::deserialize(buf, is_flexible_version, version);
        let value = decode_zig_zag_32(varint.value);
        ZigZagVarInt32 { value }
    }
}

impl ToBytes for ZigZagVarInt32 {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16) {
        let zigzag = encode_zig_zag_32(self.value);
        let varint = UnsignedVarInt32::new(zigzag);
        varint.serialize(buf, is_flexible_version, version);
    }
}
impl Default for ZigZagVarInt32 {
    fn default() -> Self {
        ZigZagVarInt32 { value: 0 }
    }
}

impl From<i32> for ZigZagVarInt32 {
    fn from(from: i32) -> Self {
        ZigZagVarInt32 { value: from }
    }
}

impl From<ZigZagVarInt32> for i32 {
    fn from(from: ZigZagVarInt32) -> Self {
        from.value
    }
}
