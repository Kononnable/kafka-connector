use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each described group.
    pub groups: Vec<DescribedGroup>,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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
        let groups = Vec::<DescribedGroup>::deserialize(version, bytes);
        (
            header,
            DescribeGroupsResponse {
                throttle_time_ms,
                groups,
            },
        )
    }
}

impl FromBytes for DescribedGroup {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let group_id = String::deserialize(version, bytes);
        let group_state = String::deserialize(version, bytes);
        let protocol_type = String::deserialize(version, bytes);
        let protocol_data = String::deserialize(version, bytes);
        let members = Vec::<DescribedGroupMember>::deserialize(version, bytes);
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

impl FromBytes for DescribedGroupMember {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let member_id = String::deserialize(version, bytes);
        let client_id = String::deserialize(version, bytes);
        let client_host = String::deserialize(version, bytes);
        let member_metadata = Vec::<u8>::deserialize(version, bytes);
        let member_assignment = Vec::<u8>::deserialize(version, bytes);
        DescribedGroupMember {
            member_id,
            client_id,
            client_host,
            member_metadata,
            member_assignment,
        }
    }
}
