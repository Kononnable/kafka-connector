use bytes::{BufMut, BytesMut};

use crate::protocol::{from_bytes::FromBytes, to_bytes::ToBytes};

pub struct CompactBytes {
    value: Vec<u8>,
}
impl FromBytes for CompactBytes {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        todo!()
        // let len = unsigned_varint::decode::u32(buf).unwrap()
        // let data: [u8; 1] = [buf.next().unwrap()];
        // i8::from_be_bytes(data)
    }
}
impl ToBytes for CompactBytes {
    fn serialize(&self, buf: &mut BytesMut) {
        todo!()
        // buf.put_i32(self.len() as i32);
    }
}
