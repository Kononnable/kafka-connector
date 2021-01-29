use super::prelude::*;

pub type SaslHandshakeRequest = SaslHandshakeRequest1;
pub type SaslHandshakeResponse = SaslHandshakeResponse1;
impl ApiCall for SaslHandshakeRequest {
    type Response = SaslHandshakeResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        1
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::SaslHandshake
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&SaslHandshakeRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> SaslHandshakeResponse {
        match version {
            0 => SaslHandshakeResponse0::deserialize(buf).into(),
            1 => SaslHandshakeResponse::deserialize(buf),
            _ => SaslHandshakeResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, ToBytes)]
pub struct SaslHandshakeRequest0 {
    pub mechanism: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct SaslHandshakeRequest1 {
    pub mechanism: String,
}

#[derive(Default, Debug, FromBytes)]
pub struct SaslHandshakeResponse0 {
    pub error_code: Int16,
    pub mechanisms: Vec<String>,
}

#[derive(Default, Debug, FromBytes)]
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
