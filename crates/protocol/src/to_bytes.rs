use crate::prelude::ApiVersion;
use bytes::{BufMut, BytesMut};
use indexmap::{IndexMap, IndexSet};

// TODO: BytesMut::put_X (all alternatives) will panic if there is not enough space,
// should it be used, or should it check if buffer is large enough on each call
// same problem in serialization of records
pub trait ToBytes {
    fn serialize(&self, version: ApiVersion, bytes: &mut BytesMut);
}

impl<K, V> ToBytes for IndexMap<K, V>
where
    K: ToBytes,
    V: ToBytes,
{
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i32(self.len() as i32);

        for (key, value) in self {
            key.serialize(_version, bytes);
            value.serialize(_version, bytes);
        }
    }
}

impl<K> ToBytes for IndexSet<K>
where
    K: ToBytes,
{
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i32(self.len() as i32);

        for key in self {
            key.serialize(_version, bytes);
        }
    }
}

impl<T> ToBytes for Option<Vec<T>>
where
    T: ToBytes,
{
    fn serialize(&self, version: ApiVersion, bytes: &mut BytesMut) {
        match self {
            Some(val) => val.serialize(version, bytes),
            None => {
                bytes.put_i32(-1);
            }
        }
    }
}

impl<T> ToBytes for Vec<T>
where
    T: ToBytes,
{
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i32(self.len() as i32);

        for element in self {
            element.serialize(_version, bytes);
        }
    }
}

impl ToBytes for Option<Vec<u8>> {
    fn serialize(&self, version: ApiVersion, bytes: &mut BytesMut) {
        match self {
            Some(val) => val.serialize(version, bytes),
            None => {
                bytes.put_i32(-1);
            }
        }
    }
}

impl ToBytes for Vec<u8> {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i32(self.len() as i32);
        bytes.put_slice(self.as_slice());
    }
}

impl ToBytes for Option<String> {
    fn serialize(&self, version: ApiVersion, bytes: &mut BytesMut) {
        match self {
            Some(val) => val.serialize(version, bytes),
            None => {
                bytes.put_i16(-1);
            }
        }
    }
}

impl ToBytes for String {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i16(self.len() as i16);
        bytes.put_slice(self.as_bytes());
    }
}

impl ToBytes for bool {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i8(*self as i8);
    }
}

impl ToBytes for i8 {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i8(*self);
    }
}

impl ToBytes for i16 {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i16(*self);
    }
}

impl ToBytes for i32 {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i32(*self);
    }
}

impl ToBytes for u32 {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_u32(*self);
    }
}

impl ToBytes for i64 {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_i64(*self);
    }
}

impl ToBytes for f64 {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        bytes.put_f64(*self);
    }
}
