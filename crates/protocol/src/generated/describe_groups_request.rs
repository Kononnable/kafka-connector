use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DescribeGroupsRequest {
    /// The names of the groups to describe
    pub groups: Vec<String>,
}

impl ApiRequest for DescribeGroupsRequest {
    type Response = super::describe_groups_response::DescribeGroupsResponse;

    fn get_api_key() -> i16 {
        15
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
    }

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.groups.serialize(version, bytes);
        }
    }
}

impl Default for DescribeGroupsRequest {
    fn default() -> Self {
        Self {
            groups: Default::default(),
        }
    }
}
