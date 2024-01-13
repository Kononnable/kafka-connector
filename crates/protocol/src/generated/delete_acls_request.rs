use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteAclsRequest {
    /// The filters to use when deleting ACLs.
    pub filters: Vec<DeleteAclsFilter>,
}

#[derive(Clone, Debug)]
pub struct DeleteAclsFilter {
    /// The resource type.
    pub resource_type_filter: i8,

    /// The resource name.
    pub resource_name_filter: Option<String>,

    /// The pattern type.
    pub pattern_type_filter: i8,

    /// The principal filter, or null to accept all principals.
    pub principal_filter: Option<String>,

    /// The host filter, or null to accept all hosts.
    pub host_filter: Option<String>,

    /// The ACL operation.
    pub operation: i8,

    /// The permission type.
    pub permission_type: i8,
}

impl ApiRequest for DeleteAclsRequest {
    type Response = super::delete_acls_response::DeleteAclsResponse;

    fn get_api_key() -> i16 {
        31
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
            self.filters.serialize(version, bytes)?;
        }
        Ok(())
    }
}

impl DeleteAclsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DeleteAclsFilter {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        if version >= 0 {
            self.resource_type_filter.serialize(version, bytes)?;
        }
        if version >= 0 {
            self.resource_name_filter.serialize(version, bytes)?;
        }
        if version >= 1 {
            self.pattern_type_filter.serialize(version, bytes)?;
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

impl DeleteAclsFilter {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.resource_name_filter.is_none() && !_version >= 0 {
            return Err(SerializationError::NullValue(
                "resource_name_filter",
                _version,
                "DeleteAclsFilter",
            ));
        }
        if self.principal_filter.is_none() && !_version >= 0 {
            return Err(SerializationError::NullValue(
                "principal_filter",
                _version,
                "DeleteAclsFilter",
            ));
        }
        if self.host_filter.is_none() && !_version >= 0 {
            return Err(SerializationError::NullValue(
                "host_filter",
                _version,
                "DeleteAclsFilter",
            ));
        }
        Ok(())
    }
}

impl Default for DeleteAclsFilter {
    fn default() -> Self {
        Self {
            resource_type_filter: Default::default(),
            resource_name_filter: Default::default(),
            pattern_type_filter: 3,
            principal_filter: Default::default(),
            host_filter: Default::default(),
            operation: Default::default(),
            permission_type: Default::default(),
        }
    }
}
