use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DescribeGroupsResponse {
    pub throttle_time_ms: i32,
    pub groups: Vec<DescribedGroup>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribedGroup {
    pub error_code: i16,
    pub group_id: String,
    pub group_state: String,
    pub protocol_type: String,
    pub protocol_data: String,
    pub members: Vec<DescribedGroupMember>,
}

#[derive(Debug, Default, Clone)]
pub struct DescribedGroupMember {
    pub member_id: String,
    pub client_id: String,
    pub client_host: String,
    pub member_metadata: Vec<u8>,
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
