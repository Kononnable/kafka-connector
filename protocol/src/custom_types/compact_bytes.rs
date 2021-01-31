use bytes::{BufMut, Bytes, BytesMut};

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::unsigned_varint32::UnsignedVarInt32;

#[derive(Debug, Clone)]
pub struct CompactBytes {
    pub value: Vec<u8>,
}
impl FromBytes for CompactBytes {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let len = UnsignedVarInt32::deserialize(buf, is_flexible_version);
        CompactBytes {
            value: buf.split_to(len.value as usize).into_iter().collect(),
        }
    }
}

impl ToBytes for CompactBytes {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        let len = UnsignedVarInt32::new(self.value.len() as u32 + 1);
        len.serialize(buf, is_flexible_version);
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
