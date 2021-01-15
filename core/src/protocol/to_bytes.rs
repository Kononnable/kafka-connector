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
impl ToBytes for Option<String> {
    fn serialize(&self, buf: &mut BytesMut) {
        match &self{
            Some(str)=>{
                buf.put_i16(str.len() as i16);
                buf.put_slice(str.as_bytes());
            },
            None=>{
                buf.put_i16(-1 as i16);
            }
        }
    }
}
impl ToBytes for i16 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.clone());
    }
}
impl ToBytes for i8 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i8(self.clone());
    }
}
impl ToBytes for i32 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(self.clone());
    }
}
impl ToBytes for i64 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i64(self.clone());
    }
}
impl<T> ToBytes for Vec<T> {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(self.len() as i32);
    }
}
