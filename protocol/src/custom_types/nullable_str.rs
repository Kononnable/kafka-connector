use bytes::{BufMut, BytesMut};

use crate::to_bytes::ToBytes;

use super::unsigned_varint32::UnsignedVarInt32;

#[derive(Debug, Clone)]
pub enum NullableStr<'a> {
    Some(&'a str),
    None,
}
impl<'a> Default for NullableStr<'a> {
    fn default() -> Self {
        Self::None
    }
}

impl<'a> ToBytes for NullableStr<'a> {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        match &self {
            Self::Some(str) => str.serialize(buf, is_flexible_version),
            Self::None => match is_flexible_version {
                true => {
                    UnsignedVarInt32::new(0).serialize(buf, is_flexible_version);
                }
                false => {
                    buf.put_i16(-1_i16);
                }
            },
        }
    }
}
