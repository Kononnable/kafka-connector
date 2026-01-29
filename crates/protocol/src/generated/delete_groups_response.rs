use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The deletion results
    pub results: IndexMap<DeletableGroupResultKey, DeletableGroupResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct DeletableGroupResultKey {
    /// The group id
    pub group_id: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeletableGroupResult {
    /// The deletion error, or 0 if the deletion succeeded.
    pub error_code: Option<ApiError>,
}

impl ApiResponse for DeleteGroupsResponse {
    type Request = super::delete_groups_request::DeleteGroupsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(42)
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
        self.results.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results =
            IndexMap::<DeletableGroupResultKey, DeletableGroupResult>::deserialize(version, bytes);
        DeleteGroupsResponse {
            throttle_time_ms,
            results,
        }
    }
}

impl DeleteGroupsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.results.iter() {
            item.0.validate_fields(_version)?;
            item.1.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for IndexMap<DeletableGroupResultKey, DeletableGroupResult> {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        _bytes.put_i32(self.len() as i32);
        for (key, value) in self {
            key.group_id.serialize(version, _bytes);
            value.error_code.serialize(version, _bytes);
        }
    }
}

impl DeletableGroupResultKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl DeletableGroupResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for IndexMap<DeletableGroupResultKey, DeletableGroupResult> {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let cap: i32 = FromBytes::deserialize(version, bytes);
        let mut ret = IndexMap::with_capacity(cap as usize);
        for _ in 0..cap {
            let group_id = String::deserialize(version, bytes);
            let error_code = Option::<ApiError>::deserialize(version, bytes);
            let key = DeletableGroupResultKey { group_id };
            let value = DeletableGroupResult { error_code };
            ret.insert(key, value);
        }

        ret
    }
}
