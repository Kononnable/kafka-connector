use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct WriteTxnMarkersResponse {
    /// The results for writing makers.
    pub markers: Vec<WritableTxnMarkerResult>,
}

#[derive(Debug, Clone)]
pub struct WritableTxnMarkerResult {
    /// The current producer ID in use by the transactional ID.
    pub producer_id: i64,

    /// The results by topic.
    pub topics: Vec<WritableTxnMarkerTopicResult>,
}

#[derive(Debug, Clone)]
pub struct WritableTxnMarkerTopicResult {
    /// The topic name.
    pub name: String,

    /// The results by partition.
    pub partitions: Vec<WritableTxnMarkerPartitionResult>,
}

#[derive(Debug, Clone)]
pub struct WritableTxnMarkerPartitionResult {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for WriteTxnMarkersResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let markers = if version >= 0 {
            Vec::<WritableTxnMarkerResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (header, WriteTxnMarkersResponse { markers })
    }
}

impl Default for WriteTxnMarkersResponse {
    fn default() -> Self {
        Self {
            markers: Default::default(),
        }
    }
}

impl FromBytes for WritableTxnMarkerResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let producer_id = if version >= 0 {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<WritableTxnMarkerTopicResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        WritableTxnMarkerResult {
            producer_id,
            topics,
        }
    }
}

impl Default for WritableTxnMarkerResult {
    fn default() -> Self {
        Self {
            producer_id: Default::default(),
            topics: Default::default(),
        }
    }
}

impl FromBytes for WritableTxnMarkerTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let partitions = if version >= 0 {
            Vec::<WritableTxnMarkerPartitionResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        WritableTxnMarkerTopicResult { name, partitions }
    }
}

impl Default for WritableTxnMarkerTopicResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl FromBytes for WritableTxnMarkerPartitionResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let partition_index = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        WritableTxnMarkerPartitionResult {
            partition_index,
            error_code,
        }
    }
}

impl Default for WritableTxnMarkerPartitionResult {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            error_code: Default::default(),
        }
    }
}