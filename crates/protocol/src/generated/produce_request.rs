use super::super::prelude::*;

/// Version 1 and 2 are the same as version 0.
///
/// Version 3 adds the transactional ID, which is used for authorization when attempting to write
/// transactional data.  Version 3 also adds support for Kafka Message Format v2.
///
/// Version 4 is the same as version 3, but the requestor must be prepared to handle a
/// KAFKA_STORAGE_ERROR.
///
/// Version 5 and 6 are the same as version 3.
///
/// Starting in version 7, records can be produced using ZStandard compression.  See KIP-110.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProduceRequest {
    /// The transactional ID, or null if the producer is not transactional.
    pub transactional_id: Option<String>,

    /// The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
    pub acks: i16,

    /// The timeout to await a response in miliseconds.
    pub timeout_ms: i32,

    /// Each topic to produce to.
    pub topics: Vec<TopicProduceData>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TopicProduceData {
    /// The topic name.
    pub name: String,

    /// Each partition to produce to.
    pub partitions: Vec<PartitionProduceData>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct PartitionProduceData {
    /// The partition index.
    pub partition_index: i32,

    /// The record data to be produced.
    pub records: Option<Vec<u8>>,
}

impl ApiRequest for ProduceRequest {
    type Response = super::produce_response::ProduceResponse;

    fn get_api_key() -> i16 {
        0
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        7
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
        if version >= 3 {
            self.transactional_id.serialize(version, bytes)?;
        }
        self.acks.serialize(version, bytes)?;
        self.timeout_ms.serialize(version, bytes)?;
        self.topics.serialize(version, bytes)?;
        Ok(())
    }
}

impl ProduceRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.transactional_id.is_none() && !_version >= 3 {
            return Err(SerializationError::NullValue(
                "transactional_id",
                _version,
                "ProduceRequest",
            ));
        }
        if self.transactional_id.is_some()
            && self.transactional_id != Some(String::default())
            && _version >= 3
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "transactional_id",
                _version,
                "ProduceRequest",
            ));
        }
        if self.acks != i16::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "acks",
                _version,
                "ProduceRequest",
            ));
        }
        if self.timeout_ms != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "timeout_ms",
                _version,
                "ProduceRequest",
            ));
        }
        if self.topics != Vec::<TopicProduceData>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "topics",
                _version,
                "ProduceRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for TopicProduceData {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl TopicProduceData {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "TopicProduceData",
            ));
        }
        if self.partitions != Vec::<PartitionProduceData>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partitions",
                _version,
                "TopicProduceData",
            ));
        }
        Ok(())
    }
}

impl ToBytes for PartitionProduceData {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        self.records.serialize(version, bytes)?;
        Ok(())
    }
}

impl PartitionProduceData {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.records.is_none() {
            return Err(SerializationError::NullValue(
                "records",
                _version,
                "PartitionProduceData",
            ));
        }
        if self.partition_index != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_index",
                _version,
                "PartitionProduceData",
            ));
        }
        if self.records.is_some() && self.records != Some(Vec::<u8>::default()) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "records",
                _version,
                "PartitionProduceData",
            ));
        }
        Ok(())
    }
}
