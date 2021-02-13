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
    fn get_first_error(response: &SaslHandshakeResponse) -> Option<ApiError> {
        SaslHandshakeResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            _ => false,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                SaslHandshakeRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                SaslHandshakeRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &SaslHandshakeRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, SaslHandshakeResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => {
                SaslHandshakeResponse0::deserialize(buf, Self::is_flexible_version(version)).into()
            }
            1 => SaslHandshakeResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => SaslHandshakeResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct SaslHandshakeRequest0 {
    pub mechanism: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct SaslHandshakeRequest1 {
    pub mechanism: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SaslHandshakeResponse0 {
    pub error_code: Int16,
    pub mechanisms: Vec<String>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct SaslHandshakeResponse1 {
    pub error_code: Int16,
    pub mechanisms: Vec<String>,
}

impl From<SaslHandshakeRequest1> for SaslHandshakeRequest0 {
    fn from(latest: SaslHandshakeRequest1) -> SaslHandshakeRequest0 {
        SaslHandshakeRequest0 {
            mechanism: latest.mechanism,
        }
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

impl SaslHandshakeResponse1 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
