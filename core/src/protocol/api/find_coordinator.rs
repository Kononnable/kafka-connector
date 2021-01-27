use super::prelude::*;

pub type FindCoordinatorRequest = FindCoordinatorRequest3;
pub type FindCoordinatorResponse = FindCoordinatorResponse3;
pub fn serialize_find_coordinator_request(
    data: FindCoordinatorRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&FindCoordinatorRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&FindCoordinatorRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&FindCoordinatorRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_find_coordinator_response(
    version: i32,
    buf: &mut Bytes,
) -> FindCoordinatorResponse {
    match version {
        0 => FindCoordinatorResponse0::deserialize(buf).into(),
        1 => FindCoordinatorResponse1::deserialize(buf).into(),
        2 => FindCoordinatorResponse2::deserialize(buf).into(),
        3 => FindCoordinatorResponse::deserialize(buf),
        _ => FindCoordinatorResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct FindCoordinatorRequest0 {
    pub key: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct FindCoordinatorRequest1 {
    pub key: String,
    pub key_type: Optional<Int8>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FindCoordinatorRequest2 {
    pub key: String,
    pub key_type: Optional<Int8>,
}

#[derive(Default, Debug, ToBytes)]
pub struct FindCoordinatorRequest3 {
    pub key: CompactString,
    pub key_type: Optional<Int8>,
}

#[derive(Default, Debug, FromBytes)]
pub struct FindCoordinatorResponse0 {
    pub error_code: Int16,
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
}

#[derive(Default, Debug, FromBytes)]
pub struct FindCoordinatorResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
}

#[derive(Default, Debug, FromBytes)]
pub struct FindCoordinatorResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
}

#[derive(Default, Debug, FromBytes)]
pub struct FindCoordinatorResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub error_message: Optional<CompactNullableString>,
    pub node_id: Int32,
    pub host: CompactString,
    pub port: Int32,
}

impl TryFrom<FindCoordinatorRequest3> for FindCoordinatorRequest0 {
    type Error = Error;
    fn try_from(latest: FindCoordinatorRequest3) -> Result<Self, Self::Error> {
        if latest.key_type.is_some() {
            return Err(Error::OldKafkaVersion(
                "FindCoordinatorRequest",
                0,
                "key_type",
            ));
        }
        Ok(FindCoordinatorRequest0 {
            key: latest.key.into(),
        })
    }
}

impl TryFrom<FindCoordinatorRequest3> for FindCoordinatorRequest1 {
    type Error = Error;
    fn try_from(latest: FindCoordinatorRequest3) -> Result<Self, Self::Error> {
        Ok(FindCoordinatorRequest1 {
            key: latest.key.into(),
            key_type: latest.key_type,
        })
    }
}

impl TryFrom<FindCoordinatorRequest3> for FindCoordinatorRequest2 {
    type Error = Error;
    fn try_from(latest: FindCoordinatorRequest3) -> Result<Self, Self::Error> {
        Ok(FindCoordinatorRequest2 {
            key: latest.key.into(),
            key_type: latest.key_type,
        })
    }
}

impl From<FindCoordinatorResponse0> for FindCoordinatorResponse3 {
    fn from(older: FindCoordinatorResponse0) -> Self {
        FindCoordinatorResponse3 {
            error_code: older.error_code,
            node_id: older.node_id,
            host: older.host.into(),
            port: older.port,
            ..FindCoordinatorResponse3::default()
        }
    }
}

impl From<FindCoordinatorResponse1> for FindCoordinatorResponse3 {
    fn from(older: FindCoordinatorResponse1) -> Self {
        FindCoordinatorResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            error_message: older.error_message.map(|val| val.into()),
            node_id: older.node_id,
            host: older.host.into(),
            port: older.port,
        }
    }
}

impl From<FindCoordinatorResponse2> for FindCoordinatorResponse3 {
    fn from(older: FindCoordinatorResponse2) -> Self {
        FindCoordinatorResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            error_message: older.error_message.map(|val| val.into()),
            node_id: older.node_id,
            host: older.host.into(),
            port: older.port,
        }
    }
}
