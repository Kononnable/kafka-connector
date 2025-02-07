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

    fn get_api_key() -> ApiKey {
        ApiKey(25)
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
        self.group_id.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
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
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
