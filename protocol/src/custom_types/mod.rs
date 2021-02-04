use self::record_batch::record_batch_with_size::RecordBatchWithSize;

pub mod nullable_str;
pub mod nullable_string;
pub mod record_batch;
pub mod tag_buffer;
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
pub type Records = RecordBatchWithSize;
