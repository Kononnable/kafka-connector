use bytes::{BufMut, Bytes, BytesMut};

use crate::protocol::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::{
    compact_string::CompactString, deserialize_unsigned_varint_32, serialize_unsigned_varint_32,
};

pub struct CompactNullableString {
    pub value: Option<String>,
}
impl FromBytes for CompactNullableString {
    fn deserialize(buf: &mut Bytes) -> Self {
        let len = deserialize_unsigned_varint_32(buf);
        if len == 0 {
            return CompactNullableString { value: None };
        }
        let slice = buf.split_to(len as usize).into_iter();
        let data: Vec<u8> = slice.take(len as usize).collect();
        let value = String::from_utf8_lossy(&data).to_string();
        CompactNullableString { value: Some(value) }
    }
}

impl ToBytes for CompactNullableString {
    fn serialize(&self, buf: &mut BytesMut) {
        match &self.value {
            Some(str) => {
                serialize_unsigned_varint_32((str.len() + 1) as u32, buf);
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
