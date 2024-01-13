use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AddPartitionsToTxnRequest {
    /// The transactional id corresponding to the transaction.
    pub transactional_id: String,

    /// Current producer id in use by the transactional id.
    pub producer_id: i64,

    /// Current epoch associated with the producer id.
    pub producer_epoch: i16,

    /// The partitions to add to the transation.
    pub topics: Vec<AddPartitionsToTxnTopic>,
}

#[derive(Clone, Debug, Default)]
pub struct AddPartitionsToTxnTopic {
    /// The name of the topic.
    pub name: String,

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
        if version >= 0 {
            self.transactional_id.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.producer_id.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.producer_epoch.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.topics.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl AddPartitionsToTxnRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for AddPartitionsToTxnTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 0 {
            self.name.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl AddPartitionsToTxnTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
