use bytes::{BufMut, Bytes, BytesMut};

pub mod compact_bytes;

pub type Boolean = bool;
pub type KafkaBytes = Vec<u8>;
pub type Int8 = i8;
pub type Int16 = i16;
pub type Int32 = i32;
pub type Int64 = i64;
pub type Float64 = f64;

// TODO:
pub type CompactString = String;
pub type NullableString = String;
pub type CompactNullableString = String;
pub type CompactRecords = i64;
pub type Records = i64;

pub fn deserialize_unsigned_varint_32(buf: &mut Bytes) -> u32 {
    let mut no_of_bytes = 0;
    while unsigned_varint::decode::is_last(buf.get(no_of_bytes).copied().unwrap()) {
        no_of_bytes += 1;
    }

    let len_slice = buf.split_to(no_of_bytes + 1);
    unsigned_varint::decode::u32(&len_slice).unwrap().0 - 1
}
pub fn serialize_unsigned_varint_32(value: u32, buf: &mut BytesMut) {
    let mut t_buf = [0u8; 5];
    let len = unsigned_varint::encode::u32((value + 1) as u32, &mut t_buf);
    buf.put_slice(len);
}
