use bytes::{Bytes, BytesMut};

use super::{from_bytes::FromBytes, to_bytes::ToBytes, unsigned_varint32::UnsignedVarInt32};

#[derive(Debug, Default, Clone, Copy)]
pub struct TagBuffer {}

impl ToBytes for TagBuffer {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, _version: u16) {
        UnsignedVarInt32::new(0).serialize(buf, is_flexible_version);
    }
}

impl FromBytes for TagBuffer {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, _version: u16) -> Self {
        UnsignedVarInt32::deserialize(buf, is_flexible_version);
        Self {}
    }
}
