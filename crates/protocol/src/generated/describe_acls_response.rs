use super::super::prelude::*;

/// Version 1 adds PatternType.
/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: Option<ApiError>,

    /// The error message, or null if there was no error.
    pub error_message: Option<String>,

    /// Each Resource that is referenced in an ACL.
    pub resources: Vec<DescribeAclsResource>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeAclsResource {
    /// The resource type.
    pub r#type: i8,

    /// The resource name.
    pub name: String,

    /// The resource pattern type.
    pub pattern_type: i8,

    /// The ACLs.
    pub acls: Vec<AclDescription>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AclDescription {
    /// The ACL principal.
    pub principal: String,

    /// The ACL host.
    pub host: String,

    /// The ACL operation.
    pub operation: i8,

    /// The ACL permission type.
    pub permission_type: i8,
}

impl ApiResponse for DescribeAclsResponse {
    type Request = super::describe_acls_request::DescribeAclsRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(29)
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
        self.error_message.serialize(version, _bytes);
        self.resources.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = i32::deserialize(version, bytes);
        let error_code = Option::<ApiError>::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let resources = Vec::<DescribeAclsResource>::deserialize(version, bytes);
        DescribeAclsResponse {
            throttle_time_ms,
            error_code,
            error_message,
            resources,
        }
    }
}

impl DescribeAclsResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.resources.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for DescribeAclsResource {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.r#type.serialize(version, _bytes);
        self.name.serialize(version, _bytes);
        if version >= ApiVersion(1) {
            self.pattern_type.serialize(version, _bytes);
        }
        self.acls.serialize(version, _bytes);
    }
}

impl DescribeAclsResource {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.acls.iter() {
            item.validate_fields(_version)?;
        }
        if self.pattern_type != 3 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "pattern_type",
                *_version,
                "DescribeAclsResource",
            ));
        }
        Ok(())
    }
}

impl FromBytes for DescribeAclsResource {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let r#type = i8::deserialize(version, bytes);
        let name = String::deserialize(version, bytes);
        let pattern_type = if version >= ApiVersion(1) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let acls = Vec::<AclDescription>::deserialize(version, bytes);
        DescribeAclsResource {
            r#type,
            name,
            pattern_type,
            acls,
        }
    }
}

impl Default for DescribeAclsResource {
    fn default() -> Self {
        Self {
            r#type: Default::default(),
            name: Default::default(),
            pattern_type: 3,
            acls: Default::default(),
        }
    }
}

impl ToBytes for AclDescription {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.principal.serialize(version, _bytes);
        self.host.serialize(version, _bytes);
        self.operation.serialize(version, _bytes);
        self.permission_type.serialize(version, _bytes);
    }
}

impl AclDescription {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AclDescription {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let principal = String::deserialize(version, bytes);
        let host = String::deserialize(version, bytes);
        let operation = i8::deserialize(version, bytes);
        let permission_type = i8::deserialize(version, bytes);
        AclDescription {
            principal,
            host,
            operation,
            permission_type,
        }
    }
}
