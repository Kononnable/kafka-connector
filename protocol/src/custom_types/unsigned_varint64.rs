use bytes::{BufMut, Bytes, BytesMut};

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

pub fn deserialize_unsigned_varint_64(buf: &mut Bytes) -> u64 {
    let mut no_of_bytes = 0;
    while !unsigned_varint::decode::is_last(buf.get(no_of_bytes).copied().unwrap()) {
        no_of_bytes += 1;
    }
    assert!(no_of_bytes < 11);
    let len_slice = buf.split_to(no_of_bytes + 1);
    unsigned_varint::decode::u64(&len_slice).unwrap().0 - 1
}
pub fn serialize_unsigned_varint_64(value: u64, buf: &mut BytesMut) {
    let mut t_buf = [0u8; 10];
    let len = unsigned_varint::encode::u64((value + 1) as u64, &mut t_buf);
    buf.put_slice(len);
}

#[derive(Debug, Clone)]
pub struct UnsignedVarInt64 {
    pub value: u64,
}
impl UnsignedVarInt64 {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}
impl FromBytes for UnsignedVarInt64 {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let value = deserialize_unsigned_varint_64(buf);
        UnsignedVarInt64 { value }
    }
}

impl ToBytes for UnsignedVarInt64 {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        serialize_unsigned_varint_64(self.value, buf);
    }
}
impl Default for UnsignedVarInt64 {
    fn default() -> Self {
        UnsignedVarInt64 { value: 0 }
    }
}

impl From<u64> for UnsignedVarInt64 {
    fn from(from: u64) -> Self {
        UnsignedVarInt64 { value: from }
    }
}

impl From<UnsignedVarInt64> for u64 {
    fn from(from: UnsignedVarInt64) -> Self {
        from.value
    }
}
