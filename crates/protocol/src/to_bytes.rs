use bytes::{BufMut, BytesMut};

pub trait ToBytes {
    fn serialize(&self, version: i16, bytes: &mut BytesMut);
}

impl<T> ToBytes for Vec<T>
where
    T: ToBytes,
{
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_i32(self.len() as i32);

        for element in self {
            element.serialize(_version, bytes);
        }
    }
}

impl ToBytes for Vec<u8> {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_i32(self.len() as i32);
        bytes.put_slice(self.as_slice());
    }
}

impl ToBytes for &str {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_i16(self.len() as i16);
        bytes.put_slice(self.as_bytes());
    }
}
impl ToBytes for String {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        self.as_str().serialize(version, bytes)
    }
}

impl ToBytes for bool {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_i8(*self as i8);
    }
}
impl ToBytes for i8 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_i8(*self);
    }
}
impl ToBytes for i16 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_i16(*self);
    }
}
impl ToBytes for i32 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_i32(*self);
    }
}
impl ToBytes for u32 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_u32(*self);
    }
}
impl ToBytes for i64 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_i64(*self);
    }
}
impl ToBytes for f64 {
    fn serialize(&self, _version: i16, bytes: &mut BytesMut) {
        bytes.put_f64(*self);
    }
}
impl<T> ToBytes for Option<T>
where
    T: ToBytes + Default,
{
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        match self {
            Some(value) => T::serialize(value, version, bytes),
            None => T::serialize(&T::default(), version, bytes),
        }
    }
}
