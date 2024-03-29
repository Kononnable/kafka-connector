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

    fn get_api_key() -> i16 {
        12
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
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.group_id.serialize(version, bytes)?;
        self.generationid.serialize(version, bytes)?;
        self.member_id.serialize(version, bytes)?;
        Ok(())
    }
}

impl HeartbeatRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.group_id != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "group_id",
                _version,
                "HeartbeatRequest",
            ));
        }
        if self.generationid != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "generationid",
                _version,
                "HeartbeatRequest",
            ));
        }
        if self.member_id != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "member_id",
                _version,
                "HeartbeatRequest",
            ));
        }
        Ok(())
    }
}
