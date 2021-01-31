use super::prelude::*;

pub type FindCoordinatorRequest = FindCoordinatorRequest3;
pub type FindCoordinatorResponse = FindCoordinatorResponse3;
impl ApiCall for FindCoordinatorRequest {
    type Response = FindCoordinatorResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::FindCoordinator
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => true,
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
                FindCoordinatorRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                FindCoordinatorRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &FindCoordinatorRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &FindCoordinatorRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &FindCoordinatorRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, FindCoordinatorResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => FindCoordinatorResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => FindCoordinatorResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => FindCoordinatorResponse2::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            3 => FindCoordinatorResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => FindCoordinatorResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct FindCoordinatorRequest0 {
    pub key: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FindCoordinatorRequest1 {
    pub key: String,
    pub key_type: Optional<Int8>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FindCoordinatorRequest2 {
    pub key: String,
    pub key_type: Optional<Int8>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct FindCoordinatorRequest3 {
    pub key: String,
    pub key_type: Optional<Int8>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FindCoordinatorResponse0 {
    pub error_code: Int16,
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FindCoordinatorResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FindCoordinatorResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct FindCoordinatorResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub tag_buffer: Optional<TagBuffer>,
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
        Ok(FindCoordinatorRequest0 { key: latest.key })
    }
}

impl TryFrom<FindCoordinatorRequest3> for FindCoordinatorRequest1 {
    type Error = Error;
    fn try_from(latest: FindCoordinatorRequest3) -> Result<Self, Self::Error> {
        Ok(FindCoordinatorRequest1 {
            key: latest.key,
            key_type: latest.key_type,
        })
    }
}

impl TryFrom<FindCoordinatorRequest3> for FindCoordinatorRequest2 {
    type Error = Error;
    fn try_from(latest: FindCoordinatorRequest3) -> Result<Self, Self::Error> {
        Ok(FindCoordinatorRequest2 {
            key: latest.key,
            key_type: latest.key_type,
        })
    }
}

impl From<FindCoordinatorResponse0> for FindCoordinatorResponse3 {
    fn from(older: FindCoordinatorResponse0) -> Self {
        FindCoordinatorResponse3 {
            error_code: older.error_code,
            node_id: older.node_id,
            host: older.host,
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
            error_message: older.error_message,
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            ..FindCoordinatorResponse3::default()
        }
    }
}

impl From<FindCoordinatorResponse2> for FindCoordinatorResponse3 {
    fn from(older: FindCoordinatorResponse2) -> Self {
        FindCoordinatorResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            error_message: older.error_message,
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            ..FindCoordinatorResponse3::default()
        }
    }
}
