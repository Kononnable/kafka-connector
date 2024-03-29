use super::super::prelude::*;

/// Version 1 adds resource pattern type.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreateAclsRequest {
    /// The ACLs that we want to create.
    pub creations: Vec<CreatableAcl>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatableAcl {
    /// The type of the resource.
    pub resource_type: i8,

    /// The resource name for the ACL.
    pub resource_name: String,

    /// The pattern type for the ACL.
    pub resource_pattern_type: i8,

    /// The principal for the ACL.
    pub principal: String,

    /// The host for the ACL.
    pub host: String,

    /// The operation type for the ACL (read, write, etc.).
    pub operation: i8,

    /// The permission type for the ACL (allow, deny, etc.).
    pub permission_type: i8,
}

impl ApiRequest for CreateAclsRequest {
    type Response = super::create_acls_response::CreateAclsResponse;

    fn get_api_key() -> i16 {
        30
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
        self.creations.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreateAclsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatableAcl {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.resource_type.serialize(version, bytes)?;
        self.resource_name.serialize(version, bytes)?;
        if version >= 1 {
            self.resource_pattern_type.serialize(version, bytes)?;
        }
        self.principal.serialize(version, bytes)?;
        self.host.serialize(version, bytes)?;
        self.operation.serialize(version, bytes)?;
        self.permission_type.serialize(version, bytes)?;
        Ok(())
    }
}

impl CreatableAcl {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.resource_pattern_type != i8::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "resource_pattern_type",
                _version,
                "CreatableAcl",
            ));
        }
        Ok(())
    }
}

impl Default for CreatableAcl {
    fn default() -> Self {
        Self {
            resource_type: Default::default(),
            resource_name: Default::default(),
            resource_pattern_type: 3,
            principal: Default::default(),
            host: Default::default(),
            operation: Default::default(),
            permission_type: Default::default(),
        }
    }
}
