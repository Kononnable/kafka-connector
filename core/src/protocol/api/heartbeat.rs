use super::prelude::*;

pub type HeartbeatRequest = HeartbeatRequest4;
pub type HeartbeatResponse = HeartbeatResponse4;
pub fn serialize_heartbeat_request(
    data: HeartbeatRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&HeartbeatRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&HeartbeatRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&HeartbeatRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&HeartbeatRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_heartbeat_response(version: i32, buf: &mut Bytes) -> HeartbeatResponse {
    match version {
        0 => HeartbeatResponse0::deserialize(buf).into(),
        1 => HeartbeatResponse1::deserialize(buf).into(),
        2 => HeartbeatResponse2::deserialize(buf).into(),
        3 => HeartbeatResponse3::deserialize(buf).into(),
        4 => HeartbeatResponse::deserialize(buf),
        _ => HeartbeatResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct HeartbeatRequest0 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
}

#[derive(Default, ToBytes)]
pub struct HeartbeatRequest1 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
}

#[derive(Default, ToBytes)]
pub struct HeartbeatRequest2 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
}

#[derive(Default, ToBytes)]
pub struct HeartbeatRequest3 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: Optional<NullableString>,
}

#[derive(Default, ToBytes)]
pub struct HeartbeatRequest4 {
    pub group_id: CompactString,
    pub generation_id: Int32,
    pub member_id: CompactString,
    pub group_instance_id: Optional<CompactNullableString>,
}

#[derive(Default, FromBytes)]
pub struct HeartbeatResponse0 {
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct HeartbeatResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct HeartbeatResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct HeartbeatResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct HeartbeatResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
}

impl TryFrom<HeartbeatRequest4> for HeartbeatRequest0 {
    type Error = Error;
    fn try_from(latest: HeartbeatRequest4) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "HeartbeatRequest",
                0,
                "group_instance_id",
            ));
        }
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
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "HeartbeatRequest",
                1,
                "group_instance_id",
            ));
        }
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
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "HeartbeatRequest",
                2,
                "group_instance_id",
            ));
        }
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
            group_instance_id: latest.group_instance_id.map(|val| val),
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
        }
    }
}

impl From<HeartbeatResponse2> for HeartbeatResponse4 {
    fn from(older: HeartbeatResponse2) -> Self {
        HeartbeatResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
        }
    }
}

impl From<HeartbeatResponse3> for HeartbeatResponse4 {
    fn from(older: HeartbeatResponse3) -> Self {
        HeartbeatResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
        }
    }
}
