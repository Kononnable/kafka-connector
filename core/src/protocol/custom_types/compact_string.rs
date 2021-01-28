use bytes::{BufMut, Bytes, BytesMut};

use crate::protocol::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::unsigned_varint32::UnsignedVarInt32;

#[derive(Debug)]
pub struct CompactString {
    pub value: String,
}
impl FromBytes for CompactString {
    fn deserialize(buf: &mut Bytes) -> Self {
        let len = UnsignedVarInt32::deserialize(buf);
        let slice = buf.split_to(len.value as usize).into_iter();
        let data: Vec<u8> = slice.take(len.value as usize).collect();
        let value = String::from_utf8_lossy(&data).to_string();
        CompactString { value }
    }
}

impl ToBytes for CompactString {
    fn serialize(&self, buf: &mut BytesMut) {
        let len = UnsignedVarInt32::new(self.value.len() as u32 + 1);
        len.serialize(buf);
        buf.put_slice(&self.value.as_bytes());
    }
}
impl Default for CompactString {
    fn default() -> Self {
        CompactString {
            value: "".to_owned(),
        }
    }
}

impl From<String> for CompactString {
    fn from(from: String) -> Self {
        CompactString { value: from }
    }
}

impl From<CompactString> for String {
    fn from(from: CompactString) -> Self {
        from.value
    }
}
