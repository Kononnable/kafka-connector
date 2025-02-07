use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InitProducerIdResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The current producer id.
    pub producer_id: i64,

    /// The current epoch associated with the producer id.
    pub producer_epoch: i16,
}

impl ApiResponse for InitProducerIdResponse {
    type Request = super::init_producer_id_request::InitProducerIdRequest;

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
        self.throttle_time_ms.serialize(version, _bytes);
        self.error_code.serialize(version, _bytes);
        self.producer_id.serialize(version, _bytes);
        self.producer_epoch.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let producer_id = i64::deserialize(version, bytes);
        let producer_epoch = i16::deserialize(version, bytes);
        InitProducerIdResponse {
            throttle_time_ms,
            error_code,
            producer_id,
            producer_epoch,
        }
    }
}

impl InitProducerIdResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
