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
    pub error_code: i16,
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
        self.throttle_time_ms.serialize(version, _bytes)?;
        self.results.serialize(version, _bytes)?;
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
        Ok(())
    }
}

impl ToBytes for DeletableGroupResultKey {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.group_id.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DeletableGroupResultKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeletableGroupResultKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        DeletableGroupResultKey { group_id }
    }
}

impl ToBytes for DeletableGroupResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DeletableGroupResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeletableGroupResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        DeletableGroupResult { error_code }
    }
}
