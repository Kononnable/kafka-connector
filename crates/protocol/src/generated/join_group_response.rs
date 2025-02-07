use super::super::prelude::*;

/// Version 1 is the same as version 0.
/// Version 2 adds throttle time.
/// Starting in version 3, on quota violation, brokers send out responses before throttling.
/// Starting in version 4, the client needs to issue a second request to join group
/// with assigned id.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JoinGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The generation ID of the group.
    pub generation_id: i32,

    /// The group protocol selected by the coordinator.
    pub protocol_name: String,

    /// The leader of the group.
    pub leader: String,

    /// The member ID assigned by the group coordinator.
    pub member_id: String,

    pub members: Vec<JoinGroupResponseMember>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct JoinGroupResponseMember {
    /// The group member ID.
    pub member_id: String,

    /// The group member metadata.
    pub metadata: Vec<u8>,
}

impl ApiResponse for JoinGroupResponse {
    type Request = super::join_group_request::JoinGroupRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(11)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(4)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        if version >= ApiVersion(2) {
            self.throttle_time_ms.serialize(version, _bytes);
        }
        self.error_code.serialize(version, _bytes);
        self.generation_id.serialize(version, _bytes);
        self.protocol_name.serialize(version, _bytes);
        self.leader.serialize(version, _bytes);
        self.member_id.serialize(version, _bytes);
        self.members.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let throttle_time_ms = if version >= ApiVersion(2) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = i16::deserialize(version, bytes);
        let generation_id = i32::deserialize(version, bytes);
        let protocol_name = String::deserialize(version, bytes);
        let leader = String::deserialize(version, bytes);
        let member_id = String::deserialize(version, bytes);
        let members = Vec::<JoinGroupResponseMember>::deserialize(version, bytes);
        JoinGroupResponse {
            throttle_time_ms,
            error_code,
            generation_id,
            protocol_name,
            leader,
            member_id,
            members,
        }
    }
}

impl JoinGroupResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.members.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for JoinGroupResponseMember {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.member_id.serialize(version, _bytes);
        self.metadata.serialize(version, _bytes);
    }
}

impl JoinGroupResponseMember {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for JoinGroupResponseMember {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let member_id = String::deserialize(version, bytes);
        let metadata = Vec::<u8>::deserialize(version, bytes);
        JoinGroupResponseMember {
            member_id,
            metadata,
        }
    }
}
