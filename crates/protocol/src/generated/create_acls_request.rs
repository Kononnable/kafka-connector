use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreateAclsRequest {
    pub creations: Vec<CreatableAcl>,
}

#[derive(Debug, Default, Clone)]
pub struct CreatableAcl {
    pub resource_type: i8,
    pub resource_name: String,
    pub resource_pattern_type: i8,
    pub principal: String,
    pub host: String,
    pub operation: i8,
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.creations.serialize(version, bytes);
        }
    }
}

impl ToBytes for CreatableAcl {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.resource_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.resource_name.serialize(version, bytes);
        }
        if version >= 1 {
            self.resource_pattern_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.principal.serialize(version, bytes);
        }
        if version >= 0 {
            self.host.serialize(version, bytes);
        }
        if version >= 0 {
            self.operation.serialize(version, bytes);
        }
        if version >= 0 {
            self.permission_type.serialize(version, bytes);
        }
    }
}
