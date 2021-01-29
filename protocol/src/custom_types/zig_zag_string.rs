use bytes::{BufMut, Bytes, BytesMut};
use std::fmt::Debug;

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::zig_zag_varint32::ZigZagVarInt32;

#[derive(Debug, Default, Clone)]
pub struct ZigZagString {
    value: String,
}
impl ToBytes for ZigZagString {
    fn serialize(&self, buf: &mut BytesMut) {
        let len = ZigZagVarInt32::new(self.value.len() as i32);
        len.serialize(buf);
        buf.put_slice(&self.value.as_bytes())
    }
}

impl FromBytes for ZigZagString {
    fn deserialize(buf: &mut Bytes) -> Self {
        let len = ZigZagVarInt32::deserialize(buf);
        let slice = buf.split_to(len.value as usize).into_iter();
        let data: Vec<u8> = slice.take(len.value as usize).collect();
        let value = String::from_utf8_lossy(&data).to_string();
        Self { value }
    }
}
