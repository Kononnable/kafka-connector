use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InitProducerIdRequest {
    /// The transactional id, or null if the producer is not transactional.
    pub transactional_id: Option<String>,

    /// The time in ms to wait for before aborting idle transactions sent by this producer.
    pub transaction_timeout_ms: i32,
}

impl ApiRequest for InitProducerIdRequest {
    type Response = super::init_producer_id_response::InitProducerIdResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(22)
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
        self.transaction_timeout_ms.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let transactional_id = Option::<String>::deserialize(version, bytes);
        let transaction_timeout_ms = i32::deserialize(version, bytes);
        InitProducerIdRequest {
            transactional_id,
            transaction_timeout_ms,
        }
    }
}

impl InitProducerIdRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
