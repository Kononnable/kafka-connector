use std::fmt::Debug;

use bytes::{BufMut, BytesMut};

use crate::custom_types::unsigned_varint32::UnsignedVarInt32;

pub trait ToBytes {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16);
}
impl<T> ToBytes for Vec<T>
where
    T: ToBytes + Debug,
{
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16) {
        match is_flexible_version {
            true => {
                UnsignedVarInt32::new(self.len() as u32 + 1).serialize(
                    buf,
                    is_flexible_version,
                    version,
                );
            }
            false => {
                buf.put_i32(self.len() as i32);
            }
        }
        for element in self {
            element.serialize(buf, is_flexible_version, version);
        }
    }
}
impl ToBytes for Vec<u8> {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16) {
        match is_flexible_version {
            true => {
                UnsignedVarInt32::new(self.len() as u32 + 1).serialize(
                    buf,
                    is_flexible_version,
                    version,
                );
            }
            false => {
                buf.put_i32(self.len() as i32);
            }
        }
        buf.put_slice(self.as_slice());
    }
}
impl ToBytes for str {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16) {
        match is_flexible_version {
            true => {
                UnsignedVarInt32::new(self.len() as u32 + 1).serialize(
                    buf,
                    is_flexible_version,
                    version,
                );
            }
            false => {
                buf.put_i16(self.len() as i16);
            }
        }
        buf.put_slice(self.as_bytes());
    }
}
impl ToBytes for String {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16) {
        self.as_str().serialize(buf, is_flexible_version, version)
    }
}

impl ToBytes for bool {
    fn serialize(&self, buf: &mut BytesMut, _is_flexible_version: bool, _version: u16) {
        buf.put_i8(*self as i8);
    }
}
impl ToBytes for i8 {
    fn serialize(&self, buf: &mut BytesMut, _is_flexible_version: bool, _version: u16) {
        buf.put_i8(*self);
    }
}
impl ToBytes for i16 {
    fn serialize(&self, buf: &mut BytesMut, _is_flexible_version: bool, _version: u16) {
        buf.put_i16(*self);
    }
}
impl ToBytes for i32 {
    fn serialize(&self, buf: &mut BytesMut, _is_flexible_version: bool, _version: u16) {
        buf.put_i32(*self);
    }
}
impl ToBytes for u32 {
    fn serialize(&self, buf: &mut BytesMut, _is_flexible_version: bool, _version: u16) {
        buf.put_u32(*self);
    }
}
impl ToBytes for i64 {
    fn serialize(&self, buf: &mut BytesMut, _is_flexible_version: bool, _version: u16) {
        buf.put_i64(*self);
    }
}
impl ToBytes for f64 {
    fn serialize(&self, buf: &mut BytesMut, _is_flexible_version: bool, _version: u16) {
        buf.put_f64(*self);
    }
}
impl<T> ToBytes for Option<T>
where
    T: ToBytes + Default,
{
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16) {
        match self {
            Some(value) => T::serialize(value, buf, is_flexible_version, version),
            None => T::serialize(&T::default(), buf, is_flexible_version, version),
        }
    }
}
