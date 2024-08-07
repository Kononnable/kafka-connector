use super::super::prelude::*;

/// Version 1 adds PatternType.
/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

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
        header: &ResponseHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.throttle_time_ms.serialize(version, bytes)?;
        self.error_code.serialize(version, bytes)?;
        self.error_message.serialize(version, bytes)?;
        self.resources.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let resources = Vec::<DescribeAclsResource>::deserialize(version, bytes);
        (
            header,
            DescribeAclsResponse {
                throttle_time_ms,
                error_code,
                error_message,
                resources,
            },
        )
    }
}

impl DescribeAclsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.error_message.is_none() {
            return Err(SerializationError::NullValue(
                "error_message",
                _version,
                "DescribeAclsResponse",
            ));
        }
        Ok(())
    }
}

impl ToBytes for DescribeAclsResource {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.r#type.serialize(version, bytes)?;
        self.name.serialize(version, bytes)?;
        if version >= 1 {
            self.pattern_type.serialize(version, bytes)?;
        }
        self.acls.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribeAclsResource {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.pattern_type != i8::default() && _version >= 1 {
            return Err(SerializationError::NonIgnorableFieldSet(
                "pattern_type",
                _version,
                "DescribeAclsResource",
            ));
        }
        Ok(())
    }
}

impl FromBytes for DescribeAclsResource {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let r#type = i8::deserialize(version, bytes);
        let name = String::deserialize(version, bytes);
        let pattern_type = if version >= 1 {
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.principal.serialize(version, bytes)?;
        self.host.serialize(version, bytes)?;
        self.operation.serialize(version, bytes)?;
        self.permission_type.serialize(version, bytes)?;
        Ok(())
    }
}

impl AclDescription {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for AclDescription {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
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
