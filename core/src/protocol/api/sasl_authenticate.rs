use super::prelude::*;

pub type SaslAuthenticateRequest = SaslAuthenticateRequest2;
pub type SaslAuthenticateResponse = SaslAuthenticateResponse2;
pub fn serialize_sasl_authenticate_request(
    data: SaslAuthenticateRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&SaslAuthenticateRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&SaslAuthenticateRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_sasl_authenticate_response(
    version: i32,
    buf: &mut Bytes,
) -> SaslAuthenticateResponse {
    match version {
        0 => SaslAuthenticateResponse0::deserialize(buf).into(),
        1 => SaslAuthenticateResponse1::deserialize(buf).into(),
        2 => SaslAuthenticateResponse::deserialize(buf),
        _ => SaslAuthenticateResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct SaslAuthenticateRequest0 {
    pub auth_bytes: KafkaBytes,
}

#[derive(Default, ToBytes)]
pub struct SaslAuthenticateRequest1 {
    pub auth_bytes: KafkaBytes,
}

#[derive(Default, ToBytes)]
pub struct SaslAuthenticateRequest2 {
    pub auth_bytes: CompactBytes,
}

#[derive(Default, FromBytes)]
pub struct SaslAuthenticateResponse0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub auth_bytes: KafkaBytes,
}

#[derive(Default, FromBytes)]
pub struct SaslAuthenticateResponse1 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub auth_bytes: KafkaBytes,
    pub session_lifetime_ms: Optional<Int64>,
}

#[derive(Default, FromBytes)]
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
            error_message: older.error_message,
            auth_bytes: older.auth_bytes.into(),
            ..SaslAuthenticateResponse2::default()
        }
    }
}

impl From<SaslAuthenticateResponse1> for SaslAuthenticateResponse2 {
    fn from(older: SaslAuthenticateResponse1) -> Self {
        SaslAuthenticateResponse2 {
            error_code: older.error_code,
            error_message: older.error_message,
            auth_bytes: older.auth_bytes.into(),
            session_lifetime_ms: older.session_lifetime_ms,
        }
    }
}
