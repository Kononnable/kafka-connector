pub mod tag_buffer;
pub mod unsigned_varint32;

pub mod from_bytes;
pub mod to_bytes;

pub type Boolean = bool;
pub type KafkaBytes = Vec<u8>;
pub type Int8 = i8;
pub type Int16 = i16;
pub type Int32 = i32;
pub type Int64 = i64;
pub type Float64 = f64;

pub type NullableString = Option<String>;
pub type NullableStr<'a> = Option<&'a str>;
