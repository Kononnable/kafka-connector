use super::prelude::*;

pub type HeartbeatRequest = HeartbeatRequest4;
pub type HeartbeatResponse = HeartbeatResponse4;
impl ApiCall for HeartbeatRequest {
    type Response = HeartbeatResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        4
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::Heartbeat
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => true,
            _ => true,
        }
    }
    fn serialize(
        self,
        version: i16,
        buf: &mut BytesMut,
        correlation_id: i32,
        client_id: &str,
    ) -> Result<(), Error> {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                HeartbeatRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                HeartbeatRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &HeartbeatRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &HeartbeatRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &HeartbeatRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &HeartbeatRequest3::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, HeartbeatResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => HeartbeatResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => HeartbeatResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => HeartbeatResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => HeartbeatResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => HeartbeatResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => HeartbeatResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct HeartbeatRequest0 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct HeartbeatRequest1 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct HeartbeatRequest2 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct HeartbeatRequest3 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: Optional<NullableString>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct HeartbeatRequest4 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: Optional<NullableString>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct HeartbeatResponse0 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct HeartbeatResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct HeartbeatResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct HeartbeatResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct HeartbeatResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub tag_buffer: Optional<TagBuffer>,
}

impl TryFrom<HeartbeatRequest4> for HeartbeatRequest0 {
    type Error = Error;
    fn try_from(latest: HeartbeatRequest4) -> Result<Self, Self::Error> {
        Ok(HeartbeatRequest0 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
        })
    }
}

impl TryFrom<HeartbeatRequest4> for HeartbeatRequest1 {
    type Error = Error;
    fn try_from(latest: HeartbeatRequest4) -> Result<Self, Self::Error> {
        Ok(HeartbeatRequest1 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
        })
    }
}

impl TryFrom<HeartbeatRequest4> for HeartbeatRequest2 {
    type Error = Error;
    fn try_from(latest: HeartbeatRequest4) -> Result<Self, Self::Error> {
        Ok(HeartbeatRequest2 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
        })
    }
}

impl TryFrom<HeartbeatRequest4> for HeartbeatRequest3 {
    type Error = Error;
    fn try_from(latest: HeartbeatRequest4) -> Result<Self, Self::Error> {
        Ok(HeartbeatRequest3 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
        })
    }
}

impl From<HeartbeatResponse0> for HeartbeatResponse4 {
    fn from(older: HeartbeatResponse0) -> Self {
        HeartbeatResponse4 {
            error_code: older.error_code,
            ..HeartbeatResponse4::default()
        }
    }
}

impl From<HeartbeatResponse1> for HeartbeatResponse4 {
    fn from(older: HeartbeatResponse1) -> Self {
        HeartbeatResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            ..HeartbeatResponse4::default()
        }
    }
}

impl From<HeartbeatResponse2> for HeartbeatResponse4 {
    fn from(older: HeartbeatResponse2) -> Self {
        HeartbeatResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            ..HeartbeatResponse4::default()
        }
    }
}

impl From<HeartbeatResponse3> for HeartbeatResponse4 {
    fn from(older: HeartbeatResponse3) -> Self {
        HeartbeatResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            ..HeartbeatResponse4::default()
        }
    }
}
