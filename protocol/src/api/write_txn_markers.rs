use super::prelude::*;

pub type WriteTxnMarkersRequest = WriteTxnMarkersRequest0;
pub type WriteTxnMarkersResponse = WriteTxnMarkersResponse0;
impl ApiCall for WriteTxnMarkersRequest {
    type Response = WriteTxnMarkersResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::WriteTxnMarkers
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> WriteTxnMarkersResponse {
        match version {
            0 => WriteTxnMarkersResponse::deserialize(buf),
            _ => WriteTxnMarkersResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct WriteTxnMarkersRequest0 {
    pub markers: Vec<WriteTxnMarkersRequestMarkers0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct WriteTxnMarkersRequestMarkers0 {
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub transaction_result: Boolean,
    pub topics: Vec<WriteTxnMarkersRequestMarkersTopics0>,
    pub coordinator_epoch: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct WriteTxnMarkersRequestMarkersTopics0 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponse0 {
    pub markers: Vec<WriteTxnMarkersResponseMarkers0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkers0 {
    pub producer_id: Int64,
    pub topics: Vec<WriteTxnMarkersResponseMarkersTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkersTopics0 {
    pub name: String,
    pub partitions: Vec<WriteTxnMarkersResponseMarkersTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkersTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}
