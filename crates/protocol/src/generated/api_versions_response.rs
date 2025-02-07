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

    fn get_api_key() -> ApiKey {
        ApiKey(18)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(2)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes);
        self.api_keys.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.throttle_time_ms.serialize(version, _bytes);
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let api_keys = IndexMap::<ApiVersionsResponseKeyKey, ApiVersionsResponseKey>::deserialize(
            version, bytes,
        );
        let throttle_time_ms = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ApiVersionsResponse {
            error_code,
            api_keys,
            throttle_time_ms,
        }
    }
}

impl ApiVersionsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.api_keys.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for ApiVersionsResponseKeyKey {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.index.serialize(version, _bytes);
    }
}

impl ApiVersionsResponseKeyKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for ApiVersionsResponseKeyKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let index = i16::deserialize(version, bytes);
        ApiVersionsResponseKeyKey { index }
    }
}

impl ToBytes for ApiVersionsResponseKey {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.min_version.serialize(version, _bytes);
        self.max_version.serialize(version, _bytes);
    }
}

impl ApiVersionsResponseKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for ApiVersionsResponseKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let min_version = i16::deserialize(version, bytes);
        let max_version = i16::deserialize(version, bytes);
        ApiVersionsResponseKey {
            min_version,
            max_version,
        }
    }
}
