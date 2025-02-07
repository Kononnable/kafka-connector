use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AddPartitionsToTxnRequest {
    /// The transactional id corresponding to the transaction.
    pub transactional_id: String,

    /// Current producer id in use by the transactional id.
    pub producer_id: i64,

    /// Current epoch associated with the producer id.
    pub producer_epoch: i16,

    /// The partitions to add to the transation.
    pub topics: IndexMap<AddPartitionsToTxnTopicKey, AddPartitionsToTxnTopic>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct AddPartitionsToTxnTopicKey {
    /// The name of the topic.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AddPartitionsToTxnTopic {
    /// The partition indexes to add to the transaction
    pub partitions: Vec<i32>,
}

impl ApiRequest for AddPartitionsToTxnRequest {
    type Response = super::add_partitions_to_txn_response::AddPartitionsToTxnResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(24)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(1)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.transactional_id.serialize(version, _bytes);
        self.producer_id.serialize(version, _bytes);
        self.producer_epoch.serialize(version, _bytes);
        self.topics.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let transactional_id = String::deserialize(version, bytes);
        let producer_id = i64::deserialize(version, bytes);
        let producer_epoch = i16::deserialize(version, bytes);
        let topics = IndexMap::<AddPartitionsToTxnTopicKey, AddPartitionsToTxnTopic>::deserialize(
            version, bytes,
        );
        AddPartitionsToTxnRequest {
            transactional_id,
            producer_id,
            producer_epoch,
            topics,
        }
    }
}

impl AddPartitionsToTxnRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for AddPartitionsToTxnTopicKey {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
    }
}

impl AddPartitionsToTxnTopicKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AddPartitionsToTxnTopicKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        AddPartitionsToTxnTopicKey { name }
    }
}

impl ToBytes for AddPartitionsToTxnTopic {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.partitions.serialize(version, _bytes);
    }
}

impl AddPartitionsToTxnTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AddPartitionsToTxnTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partitions = Vec::<i32>::deserialize(version, bytes);
        AddPartitionsToTxnTopic { partitions }
    }
}
