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
        let mut buf_batches = buf.split_off(buf.len());

        0_i32.serialize(&mut buf_batches, false, 0);
        let mut size_buf = buf_batches.split();
        size_buf.clear();

        for batch in &self.batches {
            batch.serialize(&mut buf_batches, is_flexible_version, version);
        }

        let size: i32 = buf_batches.len() as i32;
        size.serialize(&mut size_buf, is_flexible_version, version);

        buf.unsplit(size_buf);
        buf.unsplit(buf_batches);
    }
}
