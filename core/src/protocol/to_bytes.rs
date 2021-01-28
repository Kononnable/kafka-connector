use std::fmt::Debug;

use bytes::{BufMut, BytesMut};

pub trait ToBytes {
    fn serialize(&self, buf: &mut BytesMut);
}
impl ToBytes for str {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.len() as i16);
        buf.put_slice(&self.as_bytes());
    }
}
impl ToBytes for String {
    fn serialize(&self, buf: &mut BytesMut) {
        self.as_str().serialize(buf)
    }
}
impl ToBytes for Option<String> {
    fn serialize(&self, buf: &mut BytesMut) {
        match &self {
            Some(str) => str.serialize(buf),
            None => {
                buf.put_i16(-1_i16);
            }
        }
    }
}
impl ToBytes for bool {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i8(*self as i8);
    }
}
impl ToBytes for i8 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i8(*self);
    }
}
impl ToBytes for i16 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(*self);
    }
}
impl ToBytes for i32 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(*self);
    }
}
impl ToBytes for i64 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i64(*self);
    }
}
impl ToBytes for f64 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f64(*self);
    }
}
impl<T> ToBytes for Vec<T>
where
    T: ToBytes + Debug,
{
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(self.len() as i32);
        for element in self {
            element.serialize(buf);
        }
    }
}
impl ToBytes for Vec<u8> {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(self.len() as i32);
        buf.put_slice(self.as_slice());
    }
}
