use bytes::{BufMut, Bytes, BytesMut};

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

pub fn deserialize_unsigned_varint_32(buf: &mut Bytes) -> u32 {
    let mut no_of_bytes = 0;
    while unsigned_varint::decode::is_last(buf.get(no_of_bytes).copied().unwrap()) {
        no_of_bytes += 1;
    }
    assert!(no_of_bytes < 6);
    let len_slice = buf.split_to(no_of_bytes + 1);
    unsigned_varint::decode::u32(&len_slice).unwrap().0
}
pub fn serialize_unsigned_varint_32(value: u32, buf: &mut BytesMut) {
    let mut t_buf = [0u8; 5];
    let len = unsigned_varint::encode::u32(value, &mut t_buf);
    println!("XX {:?} {}", len, len.len());
    buf.put_slice(len);
}

#[derive(Debug, Clone)]
pub struct UnsignedVarInt32 {
    pub value: u32,
}
impl UnsignedVarInt32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}
impl FromBytes for UnsignedVarInt32 {
    fn deserialize(buf: &mut Bytes) -> Self {
        let value = deserialize_unsigned_varint_32(buf);
        UnsignedVarInt32 { value }
    }
}

impl ToBytes for UnsignedVarInt32 {
    fn serialize(&self, buf: &mut BytesMut) {
        serialize_unsigned_varint_32(self.value, buf);
    }
}
impl Default for UnsignedVarInt32 {
    fn default() -> Self {
        UnsignedVarInt32 { value: 0 }
    }
}

impl From<u32> for UnsignedVarInt32 {
    fn from(from: u32) -> Self {
        UnsignedVarInt32 { value: from }
    }
}

impl From<UnsignedVarInt32> for u32 {
    fn from(from: UnsignedVarInt32) -> Self {
        from.value
    }
}
