pub trait FromBytes {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>;
}
impl<R> FromBytes for Vec<R>
where
    R: FromBytes,
{
    fn deserialize<T>(buf: &mut T) -> Vec<R>
    where
        T: Iterator<Item = u8>,
    {
        let len: [u8; 4] = [
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
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
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let data: [u8; 1] = [buf.next().unwrap()];
        i8::from_be_bytes(data)
    }
}
impl FromBytes for bool {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let data: [u8; 1] = [buf.next().unwrap()];
        i8::from_be_bytes(data) == 0
    }
}
impl FromBytes for i16 {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let data: [u8; 2] = [buf.next().unwrap(), buf.next().unwrap()];
        i16::from_be_bytes(data)
    }
}
impl FromBytes for i32 {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let data: [u8; 4] = [
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
        ];
        i32::from_be_bytes(data)
    }
}
impl FromBytes for i64 {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let data: [u8; 8] = [
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
        ];
        i64::from_be_bytes(data)
    }
}
impl FromBytes for f64 {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let data: [u8; 8] = [
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
        ];
        f64::from_be_bytes(data)
    }
}
impl FromBytes for String {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let len: i16 = FromBytes::deserialize(buf);
        let data: Vec<u8> = buf.take(len as usize).collect();
        String::from_utf8_lossy(&data).to_string()
    }
}

impl FromBytes for Option<String> {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let len: i16 = FromBytes::deserialize(buf);
        if len == -1 {
            return None;
        }
        let data: Vec<u8> = buf.take(len as usize).collect();
        Some(String::from_utf8_lossy(&data).to_string())
    }
}

impl FromBytes for Vec<u8> {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let len: i32 = FromBytes::deserialize(buf);
        buf.take(len as usize).collect()
    }
}
