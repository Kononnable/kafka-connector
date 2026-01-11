use crate::{ApiVersion, FromBytes, ToBytes};
use bytes::BytesMut;

// TODO: (?) move to FromBytes/ToBytes implementation, additional method argument format: old/flexible/record

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct VarInt(pub i32);
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct VarLong(pub i64);

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct VarIntBytes(pub Vec<u8>);

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct VarIntString(pub String);

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct VarIntVec<T: FromBytes + ToBytes>(pub Vec<T>);

impl FromBytes for VarInt {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let mut no_of_bytes = 0;
        while !unsigned_varint::decode::is_last(
            bytes
                .get(no_of_bytes)
                .copied()
                .expect("Data deserialization error"),
        ) {
            no_of_bytes += 1;
        }
        assert!(no_of_bytes < 6);
        let len_slice = bytes.split_to(no_of_bytes + 1);
        let v = unsigned_varint::decode::u32(&len_slice)
            .expect("Data deserialization error")
            .0;
        Self(decode_zig_zag_32(v))
    }
}

impl ToBytes for VarInt {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        let value = encode_zig_zag_32(self.0);
        let mut t_buf = [0u8; 5];
        let len = unsigned_varint::encode::u32(value, &mut t_buf);
        bytes.extend_from_slice(len);
    }
}

impl FromBytes for VarLong {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let mut no_of_bytes = 0;
        while !unsigned_varint::decode::is_last(
            bytes
                .get(no_of_bytes)
                .copied()
                .expect("Data deserialization error"),
        ) {
            no_of_bytes += 1;
        }
        assert!(no_of_bytes < 11);
        let len_slice = bytes.split_to(no_of_bytes + 1);
        Self(decode_zig_zag_64(
            unsigned_varint::decode::u64(&len_slice)
                .expect("Data deserialization error")
                .0,
        ))
    }
}

impl ToBytes for VarLong {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        let value = encode_zig_zag_64(self.0);
        let mut t_buf = [0u8; 10];
        let len = unsigned_varint::encode::u64(value, &mut t_buf);
        bytes.extend_from_slice(len);
    }
}

impl FromBytes for VarIntBytes {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let len = VarInt::deserialize(version, bytes).0;
        if len == 0 {
            return Self(vec![]);
        }
        // TODO: zero-copy, consider BytesMut growth on non-unique BytesMut
        Self(bytes.split_to(len as usize).into_iter().collect())
    }
}

impl ToBytes for VarIntBytes {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        let len = VarInt(self.0.len() as i32);
        len.serialize(ApiVersion(0), bytes);
        bytes.extend_from_slice(self.0.as_slice());
    }
}

impl FromBytes for VarIntString {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let bytes = VarIntBytes::deserialize(version, bytes);
        Self(String::from_utf8_lossy(&bytes.0).to_string())
    }
}

impl ToBytes for VarIntString {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        let string_bytes = VarIntBytes(self.0.as_bytes().to_owned());
        string_bytes.serialize(ApiVersion(0), bytes);
    }
}

impl<T: FromBytes + ToBytes> FromBytes for VarIntVec<T> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let len = VarInt::deserialize(version, bytes).0;
        if len == 0 {
            return Self(vec![]);
        }
        let mut ret = Vec::with_capacity(len as usize);
        for _i in 0..len {
            let element = FromBytes::deserialize(version, bytes);
            ret.push(element);
        }
        Self(ret)
    }
}

impl<T: FromBytes + ToBytes> ToBytes for VarIntVec<T> {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        let len = VarInt(self.0.len() as i32);
        len.serialize(ApiVersion(0), bytes);
        for element in &self.0 {
            element.serialize(ApiVersion(0), bytes);
        }
    }
}

pub fn decode_zig_zag_32(n: u32) -> i32 {
    ((n >> 1) as i32) ^ (-((n & 1) as i32))
}

pub fn encode_zig_zag_32(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}
pub fn decode_zig_zag_64(n: u64) -> i64 {
    ((n >> 1) as i64) ^ (-((n & 1) as i64))
}

pub fn encode_zig_zag_64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}
