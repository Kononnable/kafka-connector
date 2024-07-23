use super::super::prelude::*;

/// Version 1 added throttle time.
/// Starting in version 2, on quota violation, brokers send out responses before throttling.
/// ProtocolData is currently only filled in if the group state is in the Stable state.
/// N.B. If the group is in the Dead state, the members array will always be empty.
/// This is currently only provided if the group is in the Stable state.
/// This is currently only provided if the group is in the Stable state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Each described group.
    pub groups: Vec<DescribedGroup>,
}

#[derive(Clone, Debug, PartialEq, Default)]
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

#[derive(Clone, Debug, PartialEq, Default)]
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
    type Request = super::describe_groups_request::DescribeGroupsRequest;

    fn get_api_key() -> i16 {
        15
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &ResponseHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        if version >= 1 {
            self.throttle_time_ms.serialize(version, bytes)?;
        }
        self.groups.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
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

impl DescribeGroupsResponse {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DescribedGroup {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.error_code.serialize(version, bytes)?;
        self.group_id.serialize(version, bytes)?;
        self.group_state.serialize(version, bytes)?;
        self.protocol_type.serialize(version, bytes)?;
        self.protocol_data.serialize(version, bytes)?;
        self.members.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribedGroup {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribedGroup {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
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

impl ToBytes for DescribedGroupMember {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.member_id.serialize(version, bytes)?;
        self.client_id.serialize(version, bytes)?;
        self.client_host.serialize(version, bytes)?;
        self.member_metadata.serialize(version, bytes)?;
        self.member_assignment.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribedGroupMember {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribedGroupMember {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
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
