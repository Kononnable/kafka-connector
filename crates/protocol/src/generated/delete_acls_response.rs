use super::super::prelude::*;

/// Version 1 adds the resource pattern type.
/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each filter.
    pub filter_results: Vec<DeleteAclsFilterResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeleteAclsFilterResult {
    /// The error code, or 0 if the filter succeeded.
    pub error_code: i16,

    /// The error message, or null if the filter succeeded.
    pub error_message: Option<String>,

    /// The ACLs which matched this filter.
    pub matching_acls: Vec<DeleteAclsMatchingAcl>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteAclsMatchingAcl {
    /// The deletion error code, or 0 if the deletion succeeded.
    pub error_code: i16,

    /// The deletion error message, or null if the deletion succeeded.
    pub error_message: Option<String>,

    /// The ACL resource type.
    pub resource_type: i8,

    /// The ACL resource name.
    pub resource_name: String,

    /// The ACL resource pattern type.
    pub pattern_type: i8,

    /// The ACL principal.
    pub principal: String,

    /// The ACL host.
    pub host: String,

    /// The ACL operation.
    pub operation: i8,

    /// The ACL permission type.
    pub permission_type: i8,
}

impl ApiResponse for DeleteAclsResponse {
    type Request = super::delete_acls_request::DeleteAclsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(31)
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
        self.filter_results.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let filter_results = Vec::<DeleteAclsFilterResult>::deserialize(version, bytes);
        DeleteAclsResponse {
            throttle_time_ms,
            filter_results,
        }
    }
}

impl DeleteAclsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DeleteAclsFilterResult {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes)?;
        self.error_message.serialize(version, _bytes)?;
        self.matching_acls.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DeleteAclsFilterResult {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DeleteAclsFilterResult {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let matching_acls = Vec::<DeleteAclsMatchingAcl>::deserialize(version, bytes);
        DeleteAclsFilterResult {
            error_code,
            error_message,
            matching_acls,
        }
    }
}

impl ToBytes for DeleteAclsMatchingAcl {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, _bytes)?;
        self.error_message.serialize(version, _bytes)?;
        self.resource_type.serialize(version, _bytes)?;
        self.resource_name.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.pattern_type.serialize(version, _bytes)?;
        }
        self.principal.serialize(version, _bytes)?;
        self.host.serialize(version, _bytes)?;
        self.operation.serialize(version, _bytes)?;
        self.permission_type.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DeleteAclsMatchingAcl {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.pattern_type != 3 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "pattern_type",
                *_version,
                "DeleteAclsMatchingAcl",
            ));
        }
        Ok(())
    }
}

impl FromBytes for DeleteAclsMatchingAcl {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let resource_type = i8::deserialize(version, bytes);
        let resource_name = String::deserialize(version, bytes);
        let pattern_type = if version >= ApiVersion(1) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let principal = String::deserialize(version, bytes);
        let host = String::deserialize(version, bytes);
        let operation = i8::deserialize(version, bytes);
        let permission_type = i8::deserialize(version, bytes);
        DeleteAclsMatchingAcl {
            error_code,
            error_message,
            resource_type,
            resource_name,
            pattern_type,
            principal,
            host,
            operation,
            permission_type,
        }
    }
}

impl Default for DeleteAclsMatchingAcl {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            error_message: Default::default(),
            resource_type: Default::default(),
            resource_name: Default::default(),
            pattern_type: 3,
            principal: Default::default(),
            host: Default::default(),
            operation: Default::default(),
            permission_type: Default::default(),
        }
    }
}
