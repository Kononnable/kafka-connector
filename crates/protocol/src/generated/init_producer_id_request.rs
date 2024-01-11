use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct InitProducerIdRequest {
    pub transactional_id: String,
    pub transaction_timeout_ms: i32,
}

impl ApiRequest for InitProducerIdRequest {
    type Response = super::init_producer_id_response::InitProducerIdResponse;

    fn get_api_key() -> i16 {
        22
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
            self.transaction_timeout_ms.serialize(version, bytes);
        }
    }
}
