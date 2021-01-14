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
        buf.put_i16(self.len() as i16);
        buf.put_slice(&self.as_bytes());
    }
}
impl ToBytes for i16 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.clone());
    }
}
impl ToBytes for i32 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(self.clone());
    }
}
impl<T> ToBytes for Vec<T> {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(self.len() as i32);
    }
}
