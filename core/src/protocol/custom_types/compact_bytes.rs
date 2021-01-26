use bytes::{BufMut, Bytes, BytesMut};

use crate::protocol::{from_bytes::FromBytes, to_bytes::ToBytes};

pub struct CompactBytes {
    pub value: Vec<u8>,
}
impl FromBytes for CompactBytes {
    fn deserialize(buf: &mut Bytes) -> Self {
        let mut no_of_bytes = 0;
        while unsigned_varint::decode::is_last(buf.get(no_of_bytes).copied().unwrap()) {
            no_of_bytes += 1;
        }

        let len_slice = buf.split_to(no_of_bytes + 1);
        let len = unsigned_varint::decode::u32(&len_slice).unwrap().0;
        CompactBytes {
            value: buf.split_to(len as usize).into_iter().collect(),
        }
    }
}

impl ToBytes for CompactBytes {
    fn serialize(&self, buf: &mut BytesMut) {
        let mut t_buf = [0u8; 5];
        let len = unsigned_varint::encode::u32(self.value.len() as u32, &mut t_buf);
        buf.put_slice(len);
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
