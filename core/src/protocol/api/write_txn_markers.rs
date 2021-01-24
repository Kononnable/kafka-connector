use super::prelude::*;

pub type WriteTxnMarkersRequest = WriteTxnMarkersRequest0;
pub type WriteTxnMarkersResponse = WriteTxnMarkersResponse0;
pub fn serialize_write_txn_markers_request(
    data: WriteTxnMarkersRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_write_txn_markers_response<T>(
    version: i32,
    buf: &mut T,
) -> WriteTxnMarkersResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        1 => WriteTxnMarkersResponse::deserialize(buf),
        _ => WriteTxnMarkersResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct WriteTxnMarkersRequest0 {
    pub markers: Vec<WriteTxnMarkersRequestMarkers0>,
}

#[derive(Default, ToBytes)]
pub struct WriteTxnMarkersRequestMarkers0 {
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub transaction_result: Boolean,
    pub topics: Vec<WriteTxnMarkersRequestMarkersTopics0>,
    pub coordinator_epoch: Int32,
}

#[derive(Default, ToBytes)]
pub struct WriteTxnMarkersRequestMarkersTopics0 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct WriteTxnMarkersResponse0 {
    pub markers: Vec<WriteTxnMarkersResponseMarkers0>,
}

#[derive(Default, FromBytes)]
pub struct WriteTxnMarkersResponseMarkers0 {
    pub producer_id: Int64,
    pub topics: Vec<WriteTxnMarkersResponseMarkersTopics0>,
}

#[derive(Default, FromBytes)]
pub struct WriteTxnMarkersResponseMarkersTopics0 {
    pub name: String,
    pub partitions: Vec<WriteTxnMarkersResponseMarkersTopicsPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct WriteTxnMarkersResponseMarkersTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}
