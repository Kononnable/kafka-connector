use super::super::prelude::*;

/// Version 1 and version 2 are the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeartbeatRequest {
    /// The group id.
    pub group_id: String,

    /// The generation of the group.
    pub generationid: i32,

    /// The member ID.
    pub member_id: String,
}

impl ApiRequest for HeartbeatRequest {
    type Response = super::heartbeat_response::HeartbeatResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(12)
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
        self.group_id.serialize(version, _bytes)?;
        self.generationid.serialize(version, _bytes)?;
        self.member_id.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let generationid = i32::deserialize(version, bytes);
        let member_id = String::deserialize(version, bytes);
        HeartbeatRequest {
            group_id,
            generationid,
            member_id,
        }
    }
}

impl HeartbeatRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
