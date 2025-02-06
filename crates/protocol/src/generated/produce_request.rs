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

    fn get_api_key() -> ApiKey {
        ApiKey(0)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(7)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        if version >= ApiVersion(3) {
            self.transactional_id.serialize(version, _bytes)?;
        }
        self.acks.serialize(version, _bytes)?;
        self.timeout_ms.serialize(version, _bytes)?;
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let transactional_id = if version >= ApiVersion(3) {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let acks = i16::deserialize(version, bytes);
        let timeout_ms = i32::deserialize(version, bytes);
        let topics = Vec::<TopicProduceData>::deserialize(version, bytes);
        ProduceRequest {
            transactional_id,
            acks,
            timeout_ms,
            topics,
        }
    }
}

impl ProduceRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.transactional_id != Some(String::default()) && _version.0 < 3 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "transactional_id",
                *_version,
                "ProduceRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for TopicProduceData {
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

impl TopicProduceData {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for TopicProduceData {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<PartitionProduceData>::deserialize(version, bytes);
        TopicProduceData { name, partitions }
    }
}

impl ToBytes for PartitionProduceData {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, _bytes)?;
        self.records.serialize(version, _bytes)?;
        Ok(())
    }
}

impl PartitionProduceData {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for PartitionProduceData {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let records = Option::<Vec<u8>>::deserialize(version, bytes);
        PartitionProduceData {
            partition_index,
            records,
        }
    }
}
