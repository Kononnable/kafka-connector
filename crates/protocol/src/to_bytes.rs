use crate::prelude::SerializationError;
use bytes::{BufMut, BytesMut};
use indexmap::{IndexMap, IndexSet};

pub trait ToBytes {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError>;
}

impl<K, V> ToBytes for IndexMap<K, V>
    where
        K: ToBytes,
        V: ToBytes,
{
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i32(self.len() as i32);

        for (key, value) in self {
            key.serialize(_version, bytes)?;
            value.serialize(_version, bytes)?;
        }
        Ok(())
    }
}

impl<K> ToBytes for IndexSet<K>
    where
        K: ToBytes,
{
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i32(self.len() as i32);

        for key in self {
            key.serialize(_version, bytes)?;
        }
        Ok(())
    }
}

impl<T> ToBytes for Option<Vec<T>>
    where
        T: ToBytes {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        match self {
            Some(val) => val.serialize(version, bytes),
            None => {
                bytes.put_i32(-1);
                Ok(())
            }
        }
    }
}

impl<T> ToBytes for Vec<T>
    where
        T: ToBytes,
{
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i32(self.len() as i32);

        for element in self {
            element.serialize(_version, bytes)?;
        }
        Ok(())
    }
}

impl ToBytes for Option<Vec<u8>>
{
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        match self {
            Some(val) => val.serialize(version, bytes),
            None => {
                bytes.put_i32(-1);
                Ok(())
            }
        }
    }
}

impl ToBytes for Vec<u8> {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i32(self.len() as i32);
        bytes.put_slice(self.as_slice());
        Ok(())
    }
}

impl ToBytes for Option<String> {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        match self {
            Some(val) => val.serialize(version, bytes),
            None => {
                bytes.put_i16(-1);
                Ok(())
            }
        }
    }
}

impl ToBytes for String {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i16(self.len() as i16);
        bytes.put_slice(self.as_bytes());
        Ok(())
    }
}

impl ToBytes for bool {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i8(*self as i8);
        Ok(())
    }
}

impl ToBytes for i8 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i8(*self);
        Ok(())
    }
}

impl ToBytes for i16 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i16(*self);
        Ok(())
    }
}

impl ToBytes for i32 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i32(*self);
        Ok(())
    }
}

impl ToBytes for u32 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_u32(*self);
        Ok(())
    }
}

impl ToBytes for i64 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_i64(*self);
        Ok(())
    }
}

impl ToBytes for f64 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        bytes.put_f64(*self);
        Ok(())
    }
}

