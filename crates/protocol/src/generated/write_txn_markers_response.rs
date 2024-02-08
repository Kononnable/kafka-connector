use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct WriteTxnMarkersResponse {
    /// The results for writing makers.
    pub markers: Vec<WritableTxnMarkerResult>,
}

#[derive(Clone, Debug, Default)]
pub struct WritableTxnMarkerResult {
    /// The current producer ID in use by the transactional ID.
    pub producer_id: i64,

    /// The results by topic.
    pub topics: Vec<WritableTxnMarkerTopicResult>,
}

#[derive(Clone, Debug, Default)]
pub struct WritableTxnMarkerTopicResult {
    /// The topic name.
    pub name: String,

    /// The results by partition.
    pub partitions: Vec<WritableTxnMarkerPartitionResult>,
}

#[derive(Clone, Debug, Default)]
pub struct WritableTxnMarkerPartitionResult {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for WriteTxnMarkersResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let markers = Vec::<WritableTxnMarkerResult>::deserialize(version, bytes);
        (header, WriteTxnMarkersResponse { markers })
    }
}

impl FromBytes for WritableTxnMarkerResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let producer_id = i64::deserialize(version, bytes);
        let topics = Vec::<WritableTxnMarkerTopicResult>::deserialize(version, bytes);
        WritableTxnMarkerResult {
            producer_id,
            topics,
        }
    }
}

impl FromBytes for WritableTxnMarkerTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<WritableTxnMarkerPartitionResult>::deserialize(version, bytes);
        WritableTxnMarkerTopicResult { name, partitions }
    }
}

impl FromBytes for WritableTxnMarkerPartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        WritableTxnMarkerPartitionResult {
            partition_index,
            error_code,
        }
    }
}
