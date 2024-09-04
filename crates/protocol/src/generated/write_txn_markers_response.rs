use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WriteTxnMarkersResponse {
    /// The results for writing makers.
    pub markers: Vec<WritableTxnMarkerResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct WritableTxnMarkerResult {
    /// The current producer ID in use by the transactional ID.
    pub producer_id: i64,

    /// The results by topic.
    pub topics: Vec<WritableTxnMarkerTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct WritableTxnMarkerTopicResult {
    /// The topic name.
    pub name: String,

    /// The results by partition.
    pub partitions: Vec<WritableTxnMarkerPartitionResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct WritableTxnMarkerPartitionResult {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiResponse for WriteTxnMarkersResponse {
    type Request = super::write_txn_markers_request::WriteTxnMarkersRequest;

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
        self.markers.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let markers = Vec::<WritableTxnMarkerResult>::deserialize(version, bytes);
        WriteTxnMarkersResponse { markers }
    }
}

impl WriteTxnMarkersResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for WritableTxnMarkerResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.producer_id.serialize(version, _bytes)?;
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }
}

impl WritableTxnMarkerResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for WritableTxnMarkerResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let producer_id = i64::deserialize(version, bytes);
        let topics = Vec::<WritableTxnMarkerTopicResult>::deserialize(version, bytes);
        WritableTxnMarkerResult {
            producer_id,
            topics,
        }
    }
}

impl ToBytes for WritableTxnMarkerTopicResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        self.partitions.serialize(version, _bytes)?;
        Ok(())
    }
}

impl WritableTxnMarkerTopicResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for WritableTxnMarkerTopicResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<WritableTxnMarkerPartitionResult>::deserialize(version, bytes);
        WritableTxnMarkerTopicResult { name, partitions }
    }
}

impl ToBytes for WritableTxnMarkerPartitionResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, _bytes)?;
        self.error_code.serialize(version, _bytes)?;
        Ok(())
    }
}

impl WritableTxnMarkerPartitionResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for WritableTxnMarkerPartitionResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        WritableTxnMarkerPartitionResult {
            partition_index,
            error_code,
        }
    }
}
