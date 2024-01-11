use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AddPartitionsToTxnRequest {
    pub transactional_id: String,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub topics: Vec<AddPartitionsToTxnTopic>,
}

#[derive(Debug, Default, Clone)]
pub struct AddPartitionsToTxnTopic {
    pub name: String,
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
            self.topics.serialize(version, bytes);
        }
    }
}

impl ToBytes for AddPartitionsToTxnTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.partitions.serialize(version, bytes);
        }
    }
}
