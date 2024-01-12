use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteGroupsRequest {
    /// The group names to delete.
    pub groups_names: Vec<String>,
}

impl ApiRequest for DeleteGroupsRequest {
    type Response = super::delete_groups_response::DeleteGroupsResponse;

    fn get_api_key() -> i16 {
        42
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
            self.groups_names.serialize(version, bytes);
        }
    }
}
