use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DescribeGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each described group.
    pub groups: Vec<DescribedGroup>,
}

#[derive(Debug, Clone)]
pub struct DescribedGroup {
    /// The describe error, or 0 if there was no error.
    pub error_code: i16,

    /// The group ID string.
    pub group_id: String,

    /// The group state string, or the empty string.
    pub group_state: String,

    /// The group protocol type, or the empty string.
    pub protocol_type: String,

    /// The group protocol data, or the empty string.
    pub protocol_data: String,

    /// The group members.
    pub members: Vec<DescribedGroupMember>,
}

#[derive(Debug, Clone)]
pub struct DescribedGroupMember {
    /// The member ID assigned by the group coordinator.
    pub member_id: String,

    /// The client ID used in the member's latest join group request.
    pub client_id: String,

    /// The client host.
    pub client_host: String,

    /// The metadata corresponding to the current group protocol in use.
    pub member_metadata: Vec<u8>,

    /// The current assignment provided by the group leader.
    pub member_assignment: Vec<u8>,
}

impl ApiResponse for DescribeGroupsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let groups = if version >= 0 {
            Vec::<DescribedGroup>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            DescribeGroupsResponse {
                throttle_time_ms,
                groups,
            },
        )
    }
}

impl Default for DescribeGroupsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            groups: Default::default(),
        }
    }
}

impl FromBytes for DescribedGroup {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let group_id = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let group_state = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let protocol_type = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let protocol_data = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let members = if version >= 0 {
            Vec::<DescribedGroupMember>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribedGroup {
            error_code,
            group_id,
            group_state,
            protocol_type,
            protocol_data,
            members,
        }
    }
}

impl Default for DescribedGroup {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            group_id: Default::default(),
            group_state: Default::default(),
            protocol_type: Default::default(),
            protocol_data: Default::default(),
            members: Default::default(),
        }
    }
}

impl FromBytes for DescribedGroupMember {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let member_id = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let client_id = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let client_host = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let member_metadata = if version >= 0 {
            Vec::<u8>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let member_assignment = if version >= 0 {
            Vec::<u8>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DescribedGroupMember {
            member_id,
            client_id,
            client_host,
            member_metadata,
            member_assignment,
        }
    }
}

impl Default for DescribedGroupMember {
    fn default() -> Self {
        Self {
            member_id: Default::default(),
            client_id: Default::default(),
            client_host: Default::default(),
            member_metadata: Default::default(),
            member_assignment: Default::default(),
        }
    }
}
