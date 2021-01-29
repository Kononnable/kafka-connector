use self::record_batch::RecordBatch;

pub mod compact_bytes;
pub mod compact_nullable_string;
pub mod compact_string;
pub mod optional;
pub mod record_batch;
pub mod unsigned_varint32;
pub mod unsigned_varint64;
pub mod zig_zag_string;
pub mod zig_zag_varint32;
pub mod zig_zag_varint64;
pub mod zig_zag_vec;

pub type Boolean = bool;
pub type KafkaBytes = Vec<u8>;
pub type Int8 = i8;
pub type Int16 = i16;
pub type Int32 = i32;
pub type Int64 = i64;
pub type Float64 = f64;
pub type NullableString = Option<String>;
pub type Records = RecordBatch;

// TODO:
pub type CompactRecords = RecordBatch;
