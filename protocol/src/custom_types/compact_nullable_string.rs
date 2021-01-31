use bytes::{BufMut, Bytes, BytesMut};

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::{compact_string::CompactString, unsigned_varint32::UnsignedVarInt32};

#[derive(Debug, Clone)]
pub struct CompactNullableString {
    pub value: Option<String>,
}
impl FromBytes for CompactNullableString {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let len = UnsignedVarInt32::deserialize(buf, is_flexible_version);
        if len.value == 0 {
            return CompactNullableString { value: None };
        }
        let slice = buf.split_to(len.value as usize).into_iter();
        let data: Vec<u8> = slice.take(len.value as usize).collect();
        let value = String::from_utf8_lossy(&data).to_string();
        CompactNullableString { value: Some(value) }
    }
}

impl ToBytes for CompactNullableString {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool) {
        match &self.value {
            Some(str) => {
                let len = UnsignedVarInt32::new(str.len() as u32 + 1);
                len.serialize(buf, is_flexible_version);
                buf.put_slice(&str.as_bytes());
            }
            None => {}
        }
    }
}
impl Default for CompactNullableString {
    fn default() -> Self {
        CompactNullableString { value: None }
    }
}

impl From<String> for CompactNullableString {
    fn from(from: String) -> Self {
        CompactNullableString { value: Some(from) }
    }
}

impl From<CompactString> for CompactNullableString {
    fn from(from: CompactString) -> Self {
        CompactNullableString {
            value: Some(from.value),
        }
    }
}

impl From<Option<String>> for CompactNullableString {
    fn from(from: Option<String>) -> Self {
        CompactNullableString { value: from }
    }
}

impl From<CompactNullableString> for Option<String> {
    fn from(from: CompactNullableString) -> Self {
        from.value
    }
}
