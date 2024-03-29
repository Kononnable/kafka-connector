use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WriteTxnMarkersRequest {
    /// The transaction markers to be written.
    pub markers: Vec<WritableTxnMarker>,
}

#[derive(Clone, Debug, PartialEq, Default)]
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

#[derive(Clone, Debug, PartialEq, Default)]
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

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.markers.serialize(version, bytes)?;
        Ok(())
    }
}

impl WriteTxnMarkersRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for WritableTxnMarker {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.producer_id.serialize(version, bytes)?;
        self.producer_epoch.serialize(version, bytes)?;
        self.transaction_result.serialize(version, bytes)?;
        self.topics.serialize(version, bytes)?;
        self.coordinator_epoch.serialize(version, bytes)?;
        Ok(())
    }
}

impl WritableTxnMarker {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for WritableTxnMarkerTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partition_indexes.serialize(version, bytes)?;
        Ok(())
    }
}

impl WritableTxnMarkerTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
