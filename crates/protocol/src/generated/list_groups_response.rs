use super::super::prelude::*;

/// Version 1 adds the throttle time.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// Each group in the response.
    pub groups: Vec<ListedGroup>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ListedGroup {
    /// The group ID.
    pub group_id: String,

    /// The group protocol type.
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
        let error_code = i16::deserialize(version, bytes);
        let groups = Vec::<ListedGroup>::deserialize(version, bytes);
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
        let group_id = String::deserialize(version, bytes);
        let protocol_type = String::deserialize(version, bytes);
        ListedGroup {
            group_id,
            protocol_type,
        }
    }
}
