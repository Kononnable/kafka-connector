use super::super::prelude::*;

#[derive(Clone, Debug)]
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

    fn get_api_key() -> i16 {
        29
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        if version >= 0 {
            self.resource_type.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.resource_name_filter.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.resource_pattern_type.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.principal_filter.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.host_filter.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.operation.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.permission_type.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl DescribeAclsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.resource_name_filter.is_none() && !_version >= 0 {
            return Err(SerializationError::NullValue(
                "resource_name_filter",
                _version,
                "DescribeAclsRequest",
            ));
        }
        if self.principal_filter.is_none() && !_version >= 0 {
            return Err(SerializationError::NullValue(
                "principal_filter",
                _version,
                "DescribeAclsRequest",
            ));
        }
        if self.host_filter.is_none() && !_version >= 0 {
            return Err(SerializationError::NullValue(
                "host_filter",
                _version,
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
