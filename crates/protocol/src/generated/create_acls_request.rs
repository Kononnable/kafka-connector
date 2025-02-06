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

    fn get_api_key() -> ApiKey {
        ApiKey(30)
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
        self.creations.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let creations = Vec::<CreatableAcl>::deserialize(version, bytes);
        CreateAclsRequest { creations }
    }
}

impl CreateAclsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for CreatableAcl {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.resource_type.serialize(version, _bytes)?;
        self.resource_name.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.resource_pattern_type.serialize(version, _bytes)?;
        }
        self.principal.serialize(version, _bytes)?;
        self.host.serialize(version, _bytes)?;
        self.operation.serialize(version, _bytes)?;
        self.permission_type.serialize(version, _bytes)?;
        Ok(())
    }
}

impl CreatableAcl {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.resource_pattern_type != 3 && _version.0 < 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "resource_pattern_type",
                *_version,
                "CreatableAcl",
            ));
        }
        Ok(())
    }
}

impl FromBytes for CreatableAcl {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let resource_type = i8::deserialize(version, bytes);
        let resource_name = String::deserialize(version, bytes);
        let resource_pattern_type = if version >= ApiVersion(1) {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let principal = String::deserialize(version, bytes);
        let host = String::deserialize(version, bytes);
        let operation = i8::deserialize(version, bytes);
        let permission_type = i8::deserialize(version, bytes);
        CreatableAcl {
            resource_type,
            resource_name,
            resource_pattern_type,
            principal,
            host,
            operation,
            permission_type,
        }
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
