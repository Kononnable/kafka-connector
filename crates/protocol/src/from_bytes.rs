use crate::prelude::ApiVersion;
use bytes::BytesMut;
use indexmap::{IndexMap, IndexSet};
use std::{hash::Hash, mem::size_of};

pub trait FromBytes {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self;
}

impl<K, V> FromBytes for IndexMap<K, V>
where
    K: FromBytes + Hash + Eq,
    V: FromBytes,
{
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        let mut ret = IndexMap::new();
        for _ in 0..cap {
            let key = FromBytes::deserialize(version, bytes);
            let value = FromBytes::deserialize(version, bytes);
            ret.insert(key, value);
        }
        ret
    }
}

impl<K> FromBytes for IndexSet<K>
where
    K: FromBytes + Hash + Eq,
{
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        let mut ret = IndexSet::new();
        for _i in 0..cap {
            let key = FromBytes::deserialize(version, bytes);
            ret.insert(key);
        }
        ret
    }
}

impl<T> FromBytes for Vec<T>
where
    T: FromBytes,
{
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        Option::<Vec<T>>::deserialize(version, bytes)
            .expect("Received null array response for non-nullable field")
    }
}

impl<T> FromBytes for Option<Vec<T>>
where
    T: FromBytes,
{
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        if cap == -1 {
            return None;
        }
        if cap == 0 {
            return Some(vec![]);
        }
        let mut ret = Vec::with_capacity(cap as usize);
        for _i in 0..cap {
            let element = FromBytes::deserialize(version, bytes);
            ret.push(element);
        }
        Some(ret)
    }
}

impl FromBytes for Vec<u8> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        Option::<Vec<u8>>::deserialize(version, bytes)
            .expect("Received null bytes response for non-nullable field")
    }
}

impl FromBytes for Option<Vec<u8>> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let len: i32 = FromBytes::deserialize(version, bytes);
        match len {
            -1 => None,
            0 => Some(vec![]),
            // TODO: zero-copy, consider BytesMut growth on non-unique BytesMut
            _ => Some(bytes.split_to(len as usize).into_iter().collect()),
        }
    }
}

impl FromBytes for String {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        Option::<String>::deserialize(version, bytes)
            .expect("Received null string response for non-nullable field")
    }
}

impl FromBytes for Option<String> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let len: i16 = FromBytes::deserialize(version, bytes);
        if len == -1 {
            return None;
        }
        let slice = bytes.split_to(len as usize).into_iter();
        // TODO: zero-copy, consider BytesMut growth on non-unique BytesMut
        let data: Vec<u8> = slice.take(len as usize).collect();
        Some(String::from_utf8_lossy(&data).to_string())
    }
}

impl FromBytes for i8 {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let data = bytes
            .split_to(size_of::<i8>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i8::from_be_bytes(data)
    }
}

impl FromBytes for bool {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        i8::deserialize(version, bytes) == 0
    }
}

impl FromBytes for i16 {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let data = bytes
            .split_to(size_of::<i16>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i16::from_be_bytes(data)
    }
}

impl FromBytes for i32 {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let data = bytes
            .split_to(size_of::<i32>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i32::from_be_bytes(data)
    }
}

impl FromBytes for u32 {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let data = bytes
            .split_to(size_of::<u32>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        u32::from_be_bytes(data)
    }
}

impl FromBytes for i64 {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let data = bytes
            .split_to(size_of::<i64>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        i64::from_be_bytes(data)
    }
}

impl FromBytes for f64 {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let data = bytes
            .split_to(size_of::<f64>())
            .as_ref()
            .try_into()
            .expect("Data deserialization error");
        f64::from_be_bytes(data)
    }
}
