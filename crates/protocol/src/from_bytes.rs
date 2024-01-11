use bytes::Bytes;
use std::mem::size_of;

pub trait FromBytes {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self;
}

impl<R> FromBytes for Vec<R>
where
    R: FromBytes,
{
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        if cap == -1 || cap == 0 {
            return vec![];
        }
        let mut ret = Vec::with_capacity(cap as usize);
        for _i in 0..cap {
            let element = FromBytes::deserialize(version, bytes);
            ret.push(element);
        }
        ret
    }
}
impl FromBytes for Vec<u8> {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let len: i32 = FromBytes::deserialize(version, bytes);
        if len == -1 || len == 0 {
            return vec![];
        }
        bytes.split_to(len as usize).into_iter().collect()
    }
}
impl FromBytes for String {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let len: i16 = FromBytes::deserialize(version, bytes);
        let slice = bytes.split_to(len as usize).into_iter();
        let data: Vec<u8> = slice.take(len as usize).collect();
        String::from_utf8_lossy(&data).to_string()
    }
}

impl FromBytes for i8 {
    fn deserialize(_version: i16, bytes: &mut Bytes) -> Self {
        let data = bytes
            .split_to(size_of::<i8>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i8::from_be_bytes(data)
    }
}
impl FromBytes for bool {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        i8::deserialize(version, bytes) == 0
    }
}
impl FromBytes for i16 {
    fn deserialize(_version: i16, bytes: &mut Bytes) -> Self {
        let data = bytes
            .split_to(size_of::<i16>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i16::from_be_bytes(data)
    }
}
impl FromBytes for i32 {
    fn deserialize(_version: i16, bytes: &mut Bytes) -> Self {
        let data = bytes
            .split_to(size_of::<i32>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i32::from_be_bytes(data)
    }
}
impl FromBytes for u32 {
    fn deserialize(_version: i16, bytes: &mut Bytes) -> Self {
        let data = bytes
            .split_to(size_of::<u32>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        u32::from_be_bytes(data)
    }
}
impl FromBytes for i64 {
    fn deserialize(_version: i16, bytes: &mut Bytes) -> Self {
        let data = bytes
            .split_to(size_of::<i64>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i64::from_be_bytes(data)
    }
}
impl FromBytes for f64 {
    fn deserialize(_version: i16, bytes: &mut Bytes) -> Self {
        let data = bytes
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
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        Some(T::deserialize(version, bytes))
    }
}