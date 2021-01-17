use crate::log;

pub trait FromBytes {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>;
}
impl<R> FromBytes for Vec<R>
where
    R: FromBytes + std::fmt::Debug,
{
    fn deserialize<T>(buf: &mut T) -> Vec<R>
    where
        T: Iterator<Item = u8>,
    {
        log!("Deserialize start Vec");
        let len: [u8; 4] = [
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
        ];
        let cap = i32::from_be_bytes(len);
        log!("Vec size {}", cap);
        let mut ret = Vec::with_capacity(cap as usize);
        for _i in 0..cap {
            log!("Start element {}", _i);
            let element = FromBytes::deserialize(buf);
            log!("Element deserialized: {:?}", element);
            ret.push(element);
        }
        log!("Deserialize end Vec");
        ret
    }
}
impl FromBytes for i8 {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        log!("Deserialize start i16");
        let data: [u8;1] = [buf.next().unwrap()];
        let x = i8::from_be_bytes(data);
        log!("{}", x);
        x
    }
}
impl FromBytes for i16 {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        log!("Deserialize start i16");
        let data: [u8; 2] = [buf.next().unwrap(), buf.next().unwrap()];
        let x = i16::from_be_bytes(data);
        log!("{}", x);
        x
    }
}
impl FromBytes for i32 {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        log!("Deserialize start i32");
        let data: [u8; 4] = [
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
        ];
        let x = i32::from_be_bytes(data);
        log!("{}", x);
        x
    }
}
impl FromBytes for i64 {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        log!("Deserialize start i64");
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
        let x = i64::from_be_bytes(data);
        log!("{}", x);
        x
    }
}
impl FromBytes for String {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        log!("Deserialize start String");
        let len: i16 = FromBytes::deserialize(buf);
        log!("String length {}", len);
        let data: Vec<u8> = buf.take(len as usize).collect();
        log!("Deserialize end String");
        let x = String::from_utf8_lossy(&data).to_string();
        log!("{}", x);
        x
    }
}

impl FromBytes for Option<String> {
    fn deserialize<T>(buf: &mut T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        log!("Deserialize start String");
        let len: i16 = FromBytes::deserialize(buf);
        log!("String length {}", len);
        if len == -1 {
            return None;
        }
        let data: Vec<u8> = buf.take(len as usize).collect();
        log!("Deserialize end String");
        let x = String::from_utf8_lossy(&data).to_string();
        log!("{}", x);
        Some(x)
    }
}
