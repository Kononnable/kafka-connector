use bytes::{Bytes, BytesMut};
use std::fmt::Debug;

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::unsigned_varint32::UnsignedVarInt32;

#[derive(Debug, Default, Clone)]
pub struct TagBuffer {}
impl ToBytes for TagBuffer {
    fn serialize(&self, buf: &mut BytesMut) {
        UnsignedVarInt32::new(0).serialize(buf);
    }
}

impl FromBytes for TagBuffer {
    fn deserialize(buf: &mut Bytes) -> Self {
        UnsignedVarInt32::deserialize(buf);
        Self {}
    }
}
