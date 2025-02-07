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

    fn get_api_key() -> ApiKey {
        ApiKey(27)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.markers.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let markers = Vec::<WritableTxnMarker>::deserialize(version, bytes);
        WriteTxnMarkersRequest { markers }
    }
}

impl WriteTxnMarkersRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.markers.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for WritableTxnMarker {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.producer_id.serialize(version, _bytes);
        self.producer_epoch.serialize(version, _bytes);
        self.transaction_result.serialize(version, _bytes);
        self.topics.serialize(version, _bytes);
        self.coordinator_epoch.serialize(version, _bytes);
    }
}

impl WritableTxnMarker {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for WritableTxnMarker {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let producer_id = i64::deserialize(version, bytes);
        let producer_epoch = i16::deserialize(version, bytes);
        let transaction_result = bool::deserialize(version, bytes);
        let topics = Vec::<WritableTxnMarkerTopic>::deserialize(version, bytes);
        let coordinator_epoch = i32::deserialize(version, bytes);
        WritableTxnMarker {
            producer_id,
            producer_epoch,
            transaction_result,
            topics,
            coordinator_epoch,
        }
    }
}

impl ToBytes for WritableTxnMarkerTopic {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.partition_indexes.serialize(version, _bytes);
    }
}

impl WritableTxnMarkerTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for WritableTxnMarkerTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partition_indexes = Vec::<i32>::deserialize(version, bytes);
        WritableTxnMarkerTopic {
            name,
            partition_indexes,
        }
    }
}
