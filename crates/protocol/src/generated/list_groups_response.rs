use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ListGroupsResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub groups: Vec<ListedGroup>,
}

#[derive(Debug, Default, Clone)]
pub struct ListedGroup {
    pub group_id: String,
    pub protocol_type: String,
}

impl ApiResponse for ListGroupsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let groups = if version >= 0 {
            Vec::<ListedGroup>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            ListGroupsResponse {
                throttle_time_ms,
                error_code,
                groups,
            },
        )
    }
}

impl FromBytes for ListedGroup {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let group_id = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let protocol_type = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ListedGroup {
            group_id,
            protocol_type,
        }
    }
}
