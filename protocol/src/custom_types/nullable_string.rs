use bytes::{BufMut, Bytes, BytesMut};

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::unsigned_varint32::UnsignedVarInt32;

#[derive(Debug, Clone)]
pub enum NullableString {
    Some(String),
    None,
}

impl Default for NullableString {
    fn default() -> Self {
        Self::None
    }
}
impl FromBytes for NullableString {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        let len: i16 = match is_flexible_version {
            true => {
                UnsignedVarInt32::deserialize(buf, is_flexible_version, version).value as i16 - 1
            }
            false => FromBytes::deserialize(buf, is_flexible_version, version),
        };
        if len == -1 {
            return Self::None;
        }
        let slice = buf.split_to(len as usize).into_iter();
        let data: Vec<u8> = slice.take(len as usize).collect();
        Self::Some(String::from_utf8_lossy(&data).to_string())
    }
}
impl ToBytes for NullableString {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16) {
        match &self {
            Self::Some(str) => str.serialize(buf, is_flexible_version, version),
            Self::None => match is_flexible_version {
                true => {
                    UnsignedVarInt32::new(0).serialize(buf, is_flexible_version, version);
                }
                false => {
                    buf.put_i16(-1_i16);
                }
            },
        }
    }
}

impl From<String> for NullableString {
    fn from(value: String) -> NullableString {
        NullableString::Some(value)
    }
}

impl From<NullableString> for Option<String> {
    fn from(value: NullableString) -> Option<String> {
        match value {
            NullableString::Some(v) => Some(v),
            NullableString::None => None,
        }
    }
}

impl From<Option<NullableString>> for NullableString {
    fn from(value: Option<NullableString>) -> NullableString {
        match value {
            Option::Some(v) => v,
            Option::None => NullableString::None,
        }
    }
}

impl From<Option<String>> for NullableString {
    fn from(value: Option<String>) -> NullableString {
        match value {
            Some(value) => NullableString::Some(value),
            None => NullableString::None,
        }
    }
}
