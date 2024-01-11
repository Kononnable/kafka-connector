use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeAclsRequest {
    pub resource_type: i8,
    pub resource_name_filter: String,
    pub resource_pattern_type: i8,
    pub principal_filter: String,
    pub host_filter: String,
    pub operation: i8,
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.resource_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.resource_name_filter.serialize(version, bytes);
        }
        if version >= 1 {
            self.resource_pattern_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.principal_filter.serialize(version, bytes);
        }
        if version >= 0 {
            self.host_filter.serialize(version, bytes);
        }
        if version >= 0 {
            self.operation.serialize(version, bytes);
        }
        if version >= 0 {
            self.permission_type.serialize(version, bytes);
        }
    }
}
