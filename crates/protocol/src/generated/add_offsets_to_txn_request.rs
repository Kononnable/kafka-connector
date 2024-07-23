use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AddOffsetsToTxnRequest {
    /// The transactional id corresponding to the transaction.
    pub transactional_id: String,

    /// Current producer id in use by the transactional id.
    pub producer_id: i64,

    /// Current epoch associated with the producer id.
    pub producer_epoch: i16,

    /// The unique group identifier.
    pub group_id: String,
}

impl ApiRequest for AddOffsetsToTxnRequest {
    type Response = super::add_offsets_to_txn_response::AddOffsetsToTxnResponse;

    fn get_api_key() -> i16 {
        25
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
        self.group_id.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let transactional_id = String::deserialize(version, bytes);
        let producer_id = i64::deserialize(version, bytes);
        let producer_epoch = i16::deserialize(version, bytes);
        let group_id = String::deserialize(version, bytes);
        AddOffsetsToTxnRequest {
            transactional_id,
            producer_id,
            producer_epoch,
            group_id,
        }
    }
}

impl AddOffsetsToTxnRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
