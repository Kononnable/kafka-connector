use super::super::prelude::*;

/// Version 1 adds the pattern type.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteAclsRequest {
    /// The filters to use when deleting ACLs.
    pub filters: Vec<DeleteAclsFilter>,
}

#[derive(Clone, Debug, PartialEq)]
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
        self.filters.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let filters = Vec::<DeleteAclsFilter>::deserialize(version, bytes);
        DeleteAclsRequest { filters }
    }
}

impl DeleteAclsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DeleteAclsFilter {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.resource_type_filter.serialize(version, _bytes)?;
        self.resource_name_filter.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.pattern_type_filter.serialize(version, _bytes)?;
        }
        self.principal_filter.serialize(version, _bytes)?;
        self.host_filter.serialize(version, _bytes)?;
        self.operation.serialize(version, _bytes)?;
        self.permission_type.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DeleteAclsFilter {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.pattern_type_filter != 3 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "pattern_type_filter",
                *_version,
                "DeleteAclsFilter",
            ));
        }
        Ok(())
    }
}

impl FromBytes for DeleteAclsFilter {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let resource_type_filter = i8::deserialize(version, bytes);
        let resource_name_filter = Option::<String>::deserialize(version, bytes);
        let pattern_type_filter = if version >= ApiVersion(1) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let principal_filter = Option::<String>::deserialize(version, bytes);
        let host_filter = Option::<String>::deserialize(version, bytes);
        let operation = i8::deserialize(version, bytes);
        let permission_type = i8::deserialize(version, bytes);
        DeleteAclsFilter {
            resource_type_filter,
            resource_name_filter,
            pattern_type_filter,
            principal_filter,
            host_filter,
            operation,
            permission_type,
        }
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
