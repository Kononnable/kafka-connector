use super::super::prelude::*;

/// Version 1 adds throttle time to the response.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApiVersionsResponse {
    /// The top-level error code.
    pub error_code: i16,

    /// The APIs supported by the broker.
    pub api_keys: IndexMap<ApiVersionsResponseKeyKey, ApiVersionsResponseKey>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct ApiVersionsResponseKeyKey {
    /// The API index.
    pub index: i16,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ApiVersionsResponseKey {
    /// The minimum supported version, inclusive.
    pub min_version: i16,

    /// The maximum supported version, inclusive.
    pub max_version: i16,
}

impl ApiResponse for ApiVersionsResponse {
    type Request = super::api_versions_request::ApiVersionsRequest;

    fn get_api_key() -> i16 {
        18
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &ResponseHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.error_code.serialize(version, bytes)?;
        self.api_keys.serialize(version, bytes)?;
        if version >= 1 {
            self.throttle_time_ms.serialize(version, bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let error_code = i16::deserialize(version, bytes);
        let api_keys = IndexMap::<ApiVersionsResponseKeyKey, ApiVersionsResponseKey>::deserialize(
            version, bytes,
        );
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            ApiVersionsResponse {
                error_code,
                api_keys,
                throttle_time_ms,
            },
        )
    }
}

impl ApiVersionsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for ApiVersionsResponseKeyKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.index.serialize(version, bytes)?;
        Ok(())
    }
}

impl ApiVersionsResponseKeyKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for ApiVersionsResponseKeyKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let index = i16::deserialize(version, bytes);
        ApiVersionsResponseKeyKey { index }
    }
}

impl ToBytes for ApiVersionsResponseKey {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.min_version.serialize(version, bytes)?;
        self.max_version.serialize(version, bytes)?;
        Ok(())
    }
}

impl ApiVersionsResponseKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for ApiVersionsResponseKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let min_version = i16::deserialize(version, bytes);
        let max_version = i16::deserialize(version, bytes);
        ApiVersionsResponseKey {
            min_version,
            max_version,
        }
    }
}
