use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.transactional_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.producer_id.serialize(version, bytes);
        }
        if version >= 0 {
            self.producer_epoch.serialize(version, bytes);
        }
        if version >= 0 {
            self.committed.serialize(version, bytes);
        }
    }
}
