use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct WriteTxnMarkersRequest {
    pub markers: Vec<WritableTxnMarker>,
}

#[derive(Debug, Default, Clone)]
pub struct WritableTxnMarker {
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub transaction_result: bool,
    pub topics: Vec<WritableTxnMarkerTopic>,
    pub coordinator_epoch: i32,
}

#[derive(Debug, Default, Clone)]
pub struct WritableTxnMarkerTopic {
    pub name: String,
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
