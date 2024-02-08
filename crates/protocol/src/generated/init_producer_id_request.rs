use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct InitProducerIdRequest {
    /// The transactional id, or null if the producer is not transactional.
    pub transactional_id: Option<String>,

    /// The time in ms to wait for before aborting idle transactions sent by this producer.
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
        self.transaction_timeout_ms.serialize(version, bytes)?;
        Ok(())
    }
}

impl InitProducerIdRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.transactional_id.is_none() {
            return Err(SerializationError::NullValue(
                "transactional_id",
                _version,
                "InitProducerIdRequest",
            ));
        }
        Ok(())
    }
}
