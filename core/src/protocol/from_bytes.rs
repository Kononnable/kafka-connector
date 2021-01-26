use bytes::Bytes;

pub trait FromBytes {
    fn deserialize(buf: &mut Bytes) -> Self;
}
impl<R> FromBytes for Vec<R>
where
    R: FromBytes,
{
    fn deserialize(buf: &mut Bytes) -> Self {
        let mut slice = buf.split_to(4).into_iter();
        let len: [u8; 4] = [
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
            slice.next().unwrap(),
        ];
        let cap = i32::from_be_bytes(len);
        let mut ret = Vec::with_capacity(cap as usize);
        for _i in 0..cap {
            let element = FromBytes::deserialize(buf);
            ret.push(element);
        }
        ret
    }
}
impl FromBytes for i8 {
    fn deserialize(buf: &mut Bytes) -> Self {
        let data: [u8; 1] = [buf.split_to(1).into_iter().next().unwrap()];
        i8::from_be_bytes(data)
    }
}
impl FromBytes for bool {
    fn deserialize(buf: &mut Bytes) -> Self {
        let data: [u8; 1] = [buf.split_to(1).into_iter().next().unwrap()];
        i8::from_be_bytes(data) == 0
    }
}
impl FromBytes for i16 {
    fn deserialize(buf: &mut Bytes) -> Self {
        let mut slice = buf.split_to(2).into_iter();
        let data: [u8; 2] = [slice.next().unwrap(), slice.next().unwrap()];
        i16::from_be_bytes(data)
    }
}
impl FromBytes for i32 {
    fn deserialize(buf: &mut Bytes) -> Self {
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
impl FromBytes for i64 {
    fn deserialize(buf: &mut Bytes) -> Self {
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
    fn deserialize(buf: &mut Bytes) -> Self {
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
impl FromBytes for String {
    fn deserialize(buf: &mut Bytes) -> Self {
        let len: i16 = FromBytes::deserialize(buf);
        let slice = buf.split_to(len as usize).into_iter();
        let data: Vec<u8> = slice.take(len as usize).collect();
        String::from_utf8_lossy(&data).to_string()
    }
}

impl FromBytes for Option<String> {
    fn deserialize(buf: &mut Bytes) -> Self {
        let len: i16 = FromBytes::deserialize(buf);
        if len == -1 {
            return None;
        }
        let slice = buf.split_to(len as usize).into_iter();
        let data: Vec<u8> = slice.take(len as usize).collect();
        Some(String::from_utf8_lossy(&data).to_string())
    }
}

impl FromBytes for Vec<u8> {
    fn deserialize(buf: &mut Bytes) -> Self {
        let len: i32 = FromBytes::deserialize(buf);
        buf.split_to(len as usize).into_iter().collect()
    }
}
