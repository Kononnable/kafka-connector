use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AddOffsetsToTxnRequest {
    pub transactional_id: String,
    pub producer_id: i64,
    pub producer_epoch: i16,
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
            self.group_id.serialize(version, bytes);
        }
    }
}
