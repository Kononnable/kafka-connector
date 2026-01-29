use super::super::prelude::*;

/// Version 1 adds the throttle time.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: Option<ApiError>,

    /// Each group in the response.
    pub groups: Vec<ListedGroup>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ListedGroup {
    /// The group ID.
    pub group_id: String,

    /// The group protocol type.
    pub protocol_type: String,
}

impl ApiResponse for ListGroupsResponse {
    type Request = super::list_groups_request::ListGroupsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(16)
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
        if version >= ApiVersion(1) {
            self.throttle_time_ms.serialize(version, _bytes);
        }
        self.error_code.serialize(version, _bytes);
        self.groups.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = Option::<ApiError>::deserialize(version, bytes);
        let groups = Vec::<ListedGroup>::deserialize(version, bytes);
        ListGroupsResponse {
            throttle_time_ms,
            error_code,
            groups,
        }
    }
}

impl ListGroupsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.groups.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for ListedGroup {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.group_id.serialize(version, _bytes);
        self.protocol_type.serialize(version, _bytes);
    }
}

impl ListedGroup {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for ListedGroup {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let protocol_type = String::deserialize(version, bytes);
        ListedGroup {
            group_id,
            protocol_type,
        }
    }
}
