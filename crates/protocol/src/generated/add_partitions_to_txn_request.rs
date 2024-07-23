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
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.transactional_id.serialize(version, bytes)?;
        self.producer_id.serialize(version, bytes)?;
        self.producer_epoch.serialize(version, bytes)?;
        self.topics.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
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
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for AddPartitionsToTxnTopicKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl AddPartitionsToTxnTopicKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AddPartitionsToTxnTopicKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        AddPartitionsToTxnTopicKey { name }
    }
}

impl ToBytes for AddPartitionsToTxnTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl AddPartitionsToTxnTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AddPartitionsToTxnTopic {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let partitions = Vec::<i32>::deserialize(version, bytes);
        AddPartitionsToTxnTopic { partitions }
    }
}
