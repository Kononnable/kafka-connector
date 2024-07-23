use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndTxnRequest {
    /// The ID of the transaction to end.
    pub transactional_id: String,

    /// The producer ID.
    pub producer_id: i64,

    /// The current epoch associated with the producer.
    pub producer_epoch: i16,

    /// True if the transaction was committed, false if it was aborted.
    pub committed: bool,
}

impl ApiRequest for EndTxnRequest {
    type Response = super::end_txn_response::EndTxnResponse;

    fn get_api_key() -> i16 {
        26
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
        self.committed.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let transactional_id = String::deserialize(version, bytes);
        let producer_id = i64::deserialize(version, bytes);
        let producer_epoch = i16::deserialize(version, bytes);
        let committed = bool::deserialize(version, bytes);
        EndTxnRequest {
            transactional_id,
            producer_id,
            producer_epoch,
            committed,
        }
    }
}

impl EndTxnRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
