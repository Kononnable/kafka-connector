use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
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
            self.group_id.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl AddOffsetsToTxnRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
