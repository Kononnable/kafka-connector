use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ListGroupsRequest {}

impl ApiRequest for ListGroupsRequest {
    type Response = super::list_groups_response::ListGroupsResponse;

    fn get_api_key() -> i16 {
        16
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
    }
}
