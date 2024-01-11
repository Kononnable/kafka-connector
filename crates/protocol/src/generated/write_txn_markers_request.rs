use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct WriteTxnMarkersRequest {
    /// The transaction markers to be written.
    pub markers: Vec<WritableTxnMarker>,
}

#[derive(Debug, Clone)]
pub struct WritableTxnMarker {
    /// The current producer ID.
    pub producer_id: i64,

    /// The current epoch associated with the producer ID.
    pub producer_epoch: i16,

    /// The result of the transaction to write to the partitions (false = ABORT, true = COMMIT).
    pub transaction_result: bool,

    /// Each topic that we want to write transaction marker(s) for.
    pub topics: Vec<WritableTxnMarkerTopic>,

    /// Epoch associated with the transaction state partition hosted by this transaction coordinator
    pub coordinator_epoch: i32,
}

#[derive(Debug, Clone)]
pub struct WritableTxnMarkerTopic {
    /// The topic name.
    pub name: String,

    /// The indexes of the partitions to write transaction markers for.
    pub partition_indexes: Vec<i32>,
}

impl ApiRequest for WriteTxnMarkersRequest {
    type Response = super::write_txn_markers_response::WriteTxnMarkersResponse;

    fn get_api_key() -> i16 {
        27
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        0
    }

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.markers.serialize(version, bytes);
        }
    }
}

impl Default for WriteTxnMarkersRequest {
    fn default() -> Self {
        Self {
            markers: Default::default(),
        }
    }
}

impl ToBytes for WritableTxnMarker {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.producer_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.producer_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.transaction_result.serialize(version, bytes);
        }
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
        if version >= 0 {
            self.coordinator_epoch.serialize(version, bytes);
        }
    }
}

impl Default for WritableTxnMarker {
    fn default() -> Self {
        Self {
            producer_id: Default::default(),
            producer_epoch: Default::default(),
            transaction_result: Default::default(),
            topics: Default::default(),
            coordinator_epoch: Default::default(),
        }
    }
}

impl ToBytes for WritableTxnMarkerTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partition_indexes.serialize(version, bytes);
        }
    }
}

impl Default for WritableTxnMarkerTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_indexes: Default::default(),
        }
    }
}
