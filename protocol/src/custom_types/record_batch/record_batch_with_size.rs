use bytes::{Bytes, BytesMut};

use crate::{from_bytes::FromBytes, to_bytes::ToBytes};

use super::batch::RecordBatch;
#[derive(Debug, Default, Clone)]
pub struct RecordBatchWithSize {
    pub batches: Vec<RecordBatch>,
}

impl FromBytes for RecordBatchWithSize {
    fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        let size: i32 = FromBytes::deserialize(buf, is_flexible_version, version);
        let mut slice = buf.split_to(size as usize);
        let mut batches = vec![];
        while !slice.is_empty() {
            batches.push(FromBytes::deserialize(
                &mut slice,
                is_flexible_version,
                version,
            ));
        }
        RecordBatchWithSize { batches }
    }
}

impl ToBytes for RecordBatchWithSize {
    fn serialize(&self, buf: &mut BytesMut, is_flexible_version: bool, version: u16) {
        let mut buffer = BytesMut::with_capacity(4096); // TODO: Change size(?)
        for batch in &self.batches {
            batch.serialize(&mut buffer, is_flexible_version, version);
        }
        let size: i32 = buffer.len() as i32;
        size.serialize(buf, is_flexible_version, version);
        buf.extend(buffer);
    }
}
