use bytes::{BufMut, Bytes, BytesMut};

use crate::protocol::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::{deserialize_unsigned_varint_32, serialize_unsigned_varint_32};

#[derive(Debug)]
pub struct CompactBytes {
    pub value: Vec<u8>,
}
impl FromBytes for CompactBytes {
    fn deserialize(buf: &mut Bytes) -> Self {
        let len = deserialize_unsigned_varint_32(buf);
        CompactBytes {
            value: buf.split_to(len as usize).into_iter().collect(),
        }
    }
}

impl ToBytes for CompactBytes {
    fn serialize(&self, buf: &mut BytesMut) {
        serialize_unsigned_varint_32((self.value.len() + 1) as u32, buf);
        buf.put_slice(self.value.as_slice());
    }
}
impl Default for CompactBytes {
    fn default() -> Self {
        CompactBytes { value: vec![] }
    }
}

impl From<Vec<u8>> for CompactBytes {
    fn from(from: Vec<u8>) -> Self {
        CompactBytes { value: from }
    }
}

impl From<CompactBytes> for Vec<u8> {
    fn from(from: CompactBytes) -> Self {
        from.value
    }
}
