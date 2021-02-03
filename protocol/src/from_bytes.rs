use std::fmt::Debug;

use bytes::Bytes;

use crate::custom_types::unsigned_varint32::UnsignedVarInt32;

pub trait FromBytes {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self;
}
impl<R> FromBytes for Vec<R>
where
    R: FromBytes + Debug,
{
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let cap: i32 = match is_flexible_version {
            true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i32 - 1,
            false => FromBytes::deserialize(buf, is_flexible_version),
        };
        if cap == -1 || cap == 0 {
            return vec![];
        }
        let mut ret = Vec::with_capacity(cap as usize);
        for _i in 0..cap {
            let element = FromBytes::deserialize(buf, is_flexible_version);
            ret.push(element);
        }
        ret
    }
}
impl FromBytes for Vec<u8> {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let len: i32 = match is_flexible_version {
            true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i32 - 1,
            false => FromBytes::deserialize(buf, is_flexible_version),
        };
        if len == -1 || len == 0 {
            return vec![];
        }
        buf.split_to(len as usize).into_iter().collect()
    }
}
impl FromBytes for String {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let len: i16 = match is_flexible_version {
            true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i16 - 1,
            false => FromBytes::deserialize(buf, is_flexible_version),
        };
        let slice = buf.split_to(len as usize).into_iter();
        let data: Vec<u8> = slice.take(len as usize).collect();
        String::from_utf8_lossy(&data).to_string()
    }
}

impl FromBytes for Option<String> {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool) -> Self {
        let len: i16 = match is_flexible_version {
            true => UnsignedVarInt32::deserialize(buf, is_flexible_version).value as i16 - 1,
            false => FromBytes::deserialize(buf, is_flexible_version),
        };
        if len == -1 {
            return None;
        }
        let slice = buf.split_to(len as usize).into_iter();
        let data: Vec<u8> = slice.take(len as usize).collect();
        Some(String::from_utf8_lossy(&data).to_string())
    }
}
impl FromBytes for i8 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool) -> Self {
        let data: [u8; 1] = [buf.split_to(1).into_iter().next().unwrap()];
        i8::from_be_bytes(data)
    }
}
impl FromBytes for bool {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool) -> Self {
        let data: [u8; 1] = [buf.split_to(1).into_iter().next().unwrap()];
        i8::from_be_bytes(data) == 0
    }
}
impl FromBytes for i16 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool) -> Self {
        let mut slice = buf.split_to(2).into_iter();
        let data: [u8; 2] = [slice.next().unwrap(), slice.next().unwrap()];
        i16::from_be_bytes(data)
    }
}
impl FromBytes for i32 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool) -> Self {
        let mut slice = buf.split_to(4).into_iter();
        let data: [u8; 4] = [
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
        ];
        i32::from_be_bytes(data)
    }
}
impl FromBytes for u32 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool) -> Self {
        let mut slice = buf.split_to(4).into_iter();
        let data: [u8; 4] = [
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
        ];
        u32::from_be_bytes(data)
    }
}
impl FromBytes for i64 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool) -> Self {
        let mut slice = buf.split_to(8).into_iter();
        let data: [u8; 8] = [
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
        ];
        i64::from_be_bytes(data)
    }
}
impl FromBytes for f64 {
    fn deserialize(buf: &mut Bytes, _is_flexible_version: bool) -> Self {
        let mut slice = buf.split_to(8).into_iter();
        let data: [u8; 8] = [
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
        ];
        f64::from_be_bytes(data)
    }
}
