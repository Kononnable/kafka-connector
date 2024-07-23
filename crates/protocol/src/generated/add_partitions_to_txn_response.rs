use super::super::prelude::*;

/// Starting in version 1, on quota violation brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AddPartitionsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub results: IndexMap<AddPartitionsToTxnTopicResultKey, AddPartitionsToTxnTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AddPartitionsToTxnTopicResultKey {
    /// The topic name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AddPartitionsToTxnTopicResult {
    /// The results for each partition
    pub results: IndexMap<AddPartitionsToTxnPartitionResultKey, AddPartitionsToTxnPartitionResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AddPartitionsToTxnPartitionResultKey {
    /// The partition indexes.
    pub partition_index: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AddPartitionsToTxnPartitionResult {
    /// The response error code.
    pub error_code: i16,
}

impl ApiResponse for AddPartitionsToTxnResponse {
    type Request = super::add_partitions_to_txn_request::AddPartitionsToTxnRequest;

    fn get_api_key() -> i16 {
        24
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &ResponseHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.throttle_time_ms.serialize(version, bytes)?;
        self.results.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = IndexMap::<AddPartitionsToTxnTopicResultKey,AddPartitionsToTxnTopicResult>::deserialize(version, bytes)
;
        (
            header,
            AddPartitionsToTxnResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl AddPartitionsToTxnResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for AddPartitionsToTxnTopicResultKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl AddPartitionsToTxnTopicResultKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AddPartitionsToTxnTopicResultKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        AddPartitionsToTxnTopicResultKey { name }
    }
}

impl ToBytes for AddPartitionsToTxnTopicResult {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.results.serialize(version, bytes)?;
        Ok(())
    }
}

impl AddPartitionsToTxnTopicResult {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AddPartitionsToTxnTopicResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let results = IndexMap::<
            AddPartitionsToTxnPartitionResultKey,
            AddPartitionsToTxnPartitionResult,
        >::deserialize(version, bytes);
        AddPartitionsToTxnTopicResult { results }
    }
}

impl ToBytes for AddPartitionsToTxnPartitionResultKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        Ok(())
    }
}

impl AddPartitionsToTxnPartitionResultKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AddPartitionsToTxnPartitionResultKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        AddPartitionsToTxnPartitionResultKey { partition_index }
    }
}

impl ToBytes for AddPartitionsToTxnPartitionResult {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, bytes)?;
        Ok(())
    }
}

impl AddPartitionsToTxnPartitionResult {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AddPartitionsToTxnPartitionResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        AddPartitionsToTxnPartitionResult { error_code }
    }
}
