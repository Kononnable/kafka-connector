use bytes::{BufMut, Bytes, BytesMut};
use std::fmt::Debug;

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::zig_zag_varint32::ZigZagVarInt32;

#[derive(Debug, Default, Clone)]
pub struct ZigZagVec<T> {
    value: Vec<T>,
}
impl<T> ToBytes for ZigZagVec<T>
where
    T: ToBytes + Debug,
{
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        let len = ZigZagVarInt32::new(self.value.len() as i32);
        len.serialize(buf, is_flexible_version);
        for element in &self.value {
            element.serialize(buf, is_flexible_version);
        }
    }
}
impl ToBytes for ZigZagVec<u8> {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        let len = ZigZagVarInt32::new(self.value.len() as i32);
        len.serialize(buf, is_flexible_version);
        buf.put_slice(self.value.as_slice());
    }
}

impl<T> FromBytes for ZigZagVec<T>
where
    T: FromBytes + Debug,
{
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let cap = ZigZagVarInt32::deserialize(buf, is_flexible_version);
        let mut ret = Vec::with_capacity(cap.value as usize);
        for _i in 0..cap.value {
            let element = FromBytes::deserialize(buf, is_flexible_version);
            ret.push(element);
        }
        Self { value: ret }
    }
}

impl FromBytes for ZigZagVec<u8> {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let len: ZigZagVarInt32 = FromBytes::deserialize(buf, is_flexible_version);
        let value = buf.split_to(len.value as usize).into_iter().collect();
        Self { value }
    }
}
