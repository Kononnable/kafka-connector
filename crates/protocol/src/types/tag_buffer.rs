use std::collections::{BTreeMap, HashMap};

use bytes::{BufMut, Bytes, BytesMut};

use super::{from_bytes::FromBytes, to_bytes::ToBytes, unsigned_varint32::UnsignedVarInt32};

#[derive(Debug, Default, Clone)]
pub struct TagBuffer {
    fields: BTreeMap<u32, Vec<u8>>,
}

impl ToBytes for TagBuffer {
    fn serialize(&self, buf: &mut BytesMut, _is_flexible_version: bool, _version: u16) {
        UnsignedVarInt32::new(self.fields.len() as u32).serialize(buf, true);
        for (k, v) in &self.fields {
            let key = UnsignedVarInt32::new(*k);
            key.serialize(buf, true);
            let len = UnsignedVarInt32::new(v.len() as u32);
            len.serialize(buf, true);
            buf.put_slice(v);
        }
    }
}

impl FromBytes for TagBuffer {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool, _version: u16) -> Self {
        let field_count = UnsignedVarInt32::deserialize(buf, true);
        let mut fields = BTreeMap::new();
        dbg!(field_count);
        for _ in 0..field_count.value {
            let key = UnsignedVarInt32::deserialize(buf, true).value;
            let len: i32 = UnsignedVarInt32::deserialize(buf, true).value as i32;
            let value = if len == 0 {
                vec![]
            } else {
                buf.split_to(len as usize).into_iter().collect()
            };
            dbg!(key);
            log::trace!("{:#02x}", Bytes::from(value.clone()));

            fields.insert(key, value);
        }
        Self { fields }
    }
}
