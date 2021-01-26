use super::prelude::*;

pub type SaslHandshakeRequest = SaslHandshakeRequest1;
pub type SaslHandshakeResponse = SaslHandshakeResponse1;
pub fn serialize_sasl_handshake_request(
    data: SaslHandshakeRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&SaslHandshakeRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_sasl_handshake_response(version: i32, buf: &mut Bytes) -> SaslHandshakeResponse {
    match version {
        0 => SaslHandshakeResponse0::deserialize(buf).into(),
        1 => SaslHandshakeResponse::deserialize(buf),
        _ => SaslHandshakeResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct SaslHandshakeRequest0 {
    pub mechanism: String,
}

#[derive(Default, ToBytes)]
pub struct SaslHandshakeRequest1 {
    pub mechanism: String,
}

#[derive(Default, FromBytes)]
pub struct SaslHandshakeResponse0 {
    pub error_code: Int16,
    pub mechanisms: Vec<String>,
}

#[derive(Default, FromBytes)]
pub struct SaslHandshakeResponse1 {
    pub error_code: Int16,
    pub mechanisms: Vec<String>,
}

impl TryFrom<SaslHandshakeRequest1> for SaslHandshakeRequest0 {
    type Error = Error;
    fn try_from(latest: SaslHandshakeRequest1) -> Result<Self, Self::Error> {
        Ok(SaslHandshakeRequest0 {
            mechanism: latest.mechanism,
        })
    }
}

impl From<SaslHandshakeResponse0> for SaslHandshakeResponse1 {
    fn from(older: SaslHandshakeResponse0) -> Self {
        SaslHandshakeResponse1 {
            error_code: older.error_code,
            mechanisms: older.mechanisms,
        }
    }
}
