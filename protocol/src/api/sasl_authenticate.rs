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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&SaslAuthenticateRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&SaslAuthenticateRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> SaslAuthenticateResponse {
        match version {
            0 => SaslAuthenticateResponse0::deserialize(buf).into(),
            1 => SaslAuthenticateResponse1::deserialize(buf).into(),
            2 => SaslAuthenticateResponse::deserialize(buf),
            _ => SaslAuthenticateResponse::deserialize(buf),
        }
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
    pub auth_bytes: CompactBytes,
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
    pub session_lifetime_ms: Optional<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SaslAuthenticateResponse2 {
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub auth_bytes: CompactBytes,
    pub session_lifetime_ms: Optional<Int64>,
}

impl TryFrom<SaslAuthenticateRequest2> for SaslAuthenticateRequest0 {
    type Error = Error;
    fn try_from(latest: SaslAuthenticateRequest2) -> Result<Self, Self::Error> {
        Ok(SaslAuthenticateRequest0 {
            auth_bytes: latest.auth_bytes.into(),
        })
    }
}

impl TryFrom<SaslAuthenticateRequest2> for SaslAuthenticateRequest1 {
    type Error = Error;
    fn try_from(latest: SaslAuthenticateRequest2) -> Result<Self, Self::Error> {
        Ok(SaslAuthenticateRequest1 {
            auth_bytes: latest.auth_bytes.into(),
        })
    }
}

impl From<SaslAuthenticateResponse0> for SaslAuthenticateResponse2 {
    fn from(older: SaslAuthenticateResponse0) -> Self {
        SaslAuthenticateResponse2 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
            auth_bytes: older.auth_bytes.into(),
            ..SaslAuthenticateResponse2::default()
        }
    }
}

impl From<SaslAuthenticateResponse1> for SaslAuthenticateResponse2 {
    fn from(older: SaslAuthenticateResponse1) -> Self {
        SaslAuthenticateResponse2 {
            error_code: older.error_code,
            error_message: older.error_message.into(),
            auth_bytes: older.auth_bytes.into(),
            session_lifetime_ms: older.session_lifetime_ms,
        }
    }
}
