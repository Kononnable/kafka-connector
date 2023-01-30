use std::{convert::TryInto, fmt::Debug, mem::size_of};

use bytes::Bytes;

use super::unsigned_varint32::UnsignedVarInt32;

// TODO: zero copy
// response can contain private Bytes and references to it(?)

pub trait FromBytes {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self;
}
impl<R> FromBytes for Vec<R>
where
    R: FromBytes + Debug,
{
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        log::trace!("{buf:#02x}");
        let cap: i32 = match is_flexible_version {
            true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i32 - 1,
            false => FromBytes::deserialize(buf, is_flexible_version, version),
        };
        if cap == -1 || cap == 0 {
            return vec![];
        }
        let mut ret = Vec::with_capacity(cap as usize);
        for _i in 0..cap {
            let element = FromBytes::deserialize(buf, is_flexible_version, version);
            ret.push(element);
        }
        ret
    }
}
impl FromBytes for Vec<u8> {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        let len: i32 = match is_flexible_version {
            true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i32 - 1,
            false => FromBytes::deserialize(buf, is_flexible_version, version),
        };
        if len == -1 || len == 0 {
            return vec![];
        }
        buf.split_to(len as usize).into_iter().collect()
    }
}
impl FromBytes for String {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        let len: i16 = match is_flexible_version {
            true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i16 - 1,
            false => FromBytes::deserialize(buf, is_flexible_version, version),
        };
        let slice = buf.split_to(len as usize).into_iter();
        let data: Vec<u8> = slice.take(len as usize).collect();
        String::from_utf8_lossy(&data).to_string()
    }
}

impl FromBytes for i8 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool, _version: u16) -> Self {
        let data = buf
            .split_to(size_of::<i8>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i8::from_be_bytes(data)
    }
}
impl FromBytes for bool {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        i8::deserialize(buf, is_flexible_version, version) == 0
    }
}
impl FromBytes for i16 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool, _version: u16) -> Self {
        let data = buf
            .split_to(size_of::<i16>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i16::from_be_bytes(data)
    }
}
impl FromBytes for i32 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool, _version: u16) -> Self {
        let data = buf
            .split_to(size_of::<i32>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i32::from_be_bytes(data)
    }
}
impl FromBytes for u32 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool, _version: u16) -> Self {
        let data = buf
            .split_to(size_of::<u32>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        u32::from_be_bytes(data)
    }
}
impl FromBytes for i64 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool, _version: u16) -> Self {
        let data = buf
            .split_to(size_of::<i64>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i64::from_be_bytes(data)
    }
}
impl FromBytes for f64 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool, _version: u16) -> Self {
        let data = buf
            .split_to(size_of::<f64>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        f64::from_be_bytes(data)
    }
}

impl<T> FromBytes for Option<T>
where
    T: FromBytes,
{
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        Some(T::deserialize(buf, is_flexible_version, version))
    }
}
