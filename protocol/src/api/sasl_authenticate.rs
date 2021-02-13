use super::prelude::*;

pub type SaslAuthenticateRequest = SaslAuthenticateRequest2;
pub type SaslAuthenticateResponse = SaslAuthenticateResponse2;
impl ApiCall for SaslAuthenticateRequest {
    type Response = SaslAuthenticateResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::SaslAuthenticate
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                SaslAuthenticateRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                SaslAuthenticateRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &SaslAuthenticateRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &SaslAuthenticateRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, SaslAuthenticateResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => SaslAuthenticateResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => SaslAuthenticateResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => SaslAuthenticateResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => SaslAuthenticateResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct SaslAuthenticateRequest0 {
    pub auth_bytes: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SaslAuthenticateRequest1 {
    pub auth_bytes: KafkaBytes,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SaslAuthenticateRequest2 {
    pub auth_bytes: KafkaBytes,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SaslAuthenticateResponse0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub auth_bytes: KafkaBytes,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SaslAuthenticateResponse1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub auth_bytes: KafkaBytes,
    pub session_lifetime_ms: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SaslAuthenticateResponse2 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub auth_bytes: KafkaBytes,
    pub session_lifetime_ms: Option<Int64>,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<SaslAuthenticateRequest2> for SaslAuthenticateRequest0 {
    fn from(latest: SaslAuthenticateRequest2) -> SaslAuthenticateRequest0 {
        SaslAuthenticateRequest0 {
            auth_bytes: latest.auth_bytes,
        }
    }
}

impl From<SaslAuthenticateRequest2> for SaslAuthenticateRequest1 {
    fn from(latest: SaslAuthenticateRequest2) -> SaslAuthenticateRequest1 {
        SaslAuthenticateRequest1 {
            auth_bytes: latest.auth_bytes,
        }
    }
}

impl From<SaslAuthenticateResponse0> for SaslAuthenticateResponse2 {
    fn from(older: SaslAuthenticateResponse0) -> Self {
        SaslAuthenticateResponse2 {
            error_code: older.error_code,
            error_message: older.error_message,
            auth_bytes: older.auth_bytes,
            ..SaslAuthenticateResponse2::default()
        }
    }
}

impl From<SaslAuthenticateResponse1> for SaslAuthenticateResponse2 {
    fn from(older: SaslAuthenticateResponse1) -> Self {
        SaslAuthenticateResponse2 {
            error_code: older.error_code,
            error_message: older.error_message,
            auth_bytes: older.auth_bytes,
            session_lifetime_ms: older.session_lifetime_ms,
            ..SaslAuthenticateResponse2::default()
        }
    }
}
