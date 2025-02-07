use super::super::prelude::*;

/// Versions 1 and 2 are the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SyncGroupRequest {
    /// The unique group identifier.
    pub group_id: String,

    /// The generation of the group.
    pub generation_id: i32,

    /// The member ID assigned by the group.
    pub member_id: String,

    /// Each assignment.
    pub assignments: Vec<SyncGroupRequestAssignment>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SyncGroupRequestAssignment {
    /// The ID of the member to assign.
    pub member_id: String,

    /// The member assignment.
    pub assignment: Vec<u8>,
}

impl ApiRequest for SyncGroupRequest {
    type Response = super::sync_group_response::SyncGroupResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(14)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(2)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.group_id.serialize(version, _bytes);
        self.generation_id.serialize(version, _bytes);
        self.member_id.serialize(version, _bytes);
        self.assignments.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let generation_id = i32::deserialize(version, bytes);
        let member_id = String::deserialize(version, bytes);
        let assignments = Vec::<SyncGroupRequestAssignment>::deserialize(version, bytes);
        SyncGroupRequest {
            group_id,
            generation_id,
            member_id,
            assignments,
        }
    }
}

impl SyncGroupRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.assignments.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for SyncGroupRequestAssignment {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.member_id.serialize(version, _bytes);
        self.assignment.serialize(version, _bytes);
    }
}

impl SyncGroupRequestAssignment {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for SyncGroupRequestAssignment {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let member_id = String::deserialize(version, bytes);
        let assignment = Vec::<u8>::deserialize(version, bytes);
        SyncGroupRequestAssignment {
            member_id,
            assignment,
        }
    }
}
