use super::super::prelude::*;

/// Version 1 adds resource pattern type.
#[derive(Clone, Debug, PartialEq)]
pub struct DescribeAclsRequest {
    /// The resource type.
    pub resource_type: i8,

    /// The resource name, or null to match any resource name.
    pub resource_name_filter: Option<String>,

    /// The resource pattern to match.
    pub resource_pattern_type: i8,

    /// The principal to match, or null to match any principal.
    pub principal_filter: Option<String>,

    /// The host to match, or null to match any host.
    pub host_filter: Option<String>,

    /// The operation to match.
    pub operation: i8,

    /// The permission type to match.
    pub permission_type: i8,
}

impl ApiRequest for DescribeAclsRequest {
    type Response = super::describe_acls_response::DescribeAclsResponse;

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
        self.resource_type.serialize(version, _bytes)?;
        self.resource_name_filter.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.resource_pattern_type.serialize(version, _bytes)?;
        }
        self.principal_filter.serialize(version, _bytes)?;
        self.host_filter.serialize(version, _bytes)?;
        self.operation.serialize(version, _bytes)?;
        self.permission_type.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let resource_type = i8::deserialize(version, bytes);
        let resource_name_filter = Option::<String>::deserialize(version, bytes);
        let resource_pattern_type = if version >= ApiVersion(1) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let principal_filter = Option::<String>::deserialize(version, bytes);
        let host_filter = Option::<String>::deserialize(version, bytes);
        let operation = i8::deserialize(version, bytes);
        let permission_type = i8::deserialize(version, bytes);
        DescribeAclsRequest {
            resource_type,
            resource_name_filter,
            resource_pattern_type,
            principal_filter,
            host_filter,
            operation,
            permission_type,
        }
    }
}

impl DescribeAclsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.resource_name_filter.is_none() {
            return Err(SerializationError::NullValue(
                "resource_name_filter",
                *_version,
                "DescribeAclsRequest",
            ));
        }
        if self.principal_filter.is_none() {
            return Err(SerializationError::NullValue(
                "principal_filter",
                *_version,
                "DescribeAclsRequest",
            ));
        }
        if self.host_filter.is_none() {
            return Err(SerializationError::NullValue(
                "host_filter",
                *_version,
                "DescribeAclsRequest",
            ));
        }
        if self.resource_pattern_type != i8::default() && _version >= ApiVersion(1) {
            return Err(SerializationError::NonIgnorableFieldSet(
                "resource_pattern_type",
                *_version,
                "DescribeAclsRequest",
            ));
        }
        Ok(())
    }
}

impl Default for DescribeAclsRequest {
    fn default() -> Self {
        Self {
            resource_type: Default::default(),
            resource_name_filter: Default::default(),
            resource_pattern_type: 3,
            principal_filter: Default::default(),
            host_filter: Default::default(),
            operation: Default::default(),
            permission_type: Default::default(),
        }
    }
}
