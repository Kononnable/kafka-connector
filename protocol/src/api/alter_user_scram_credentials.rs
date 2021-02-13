use super::prelude::*;

pub type AlterUserScramCredentialsRequest = AlterUserScramCredentialsRequest0;
pub type AlterUserScramCredentialsResponse = AlterUserScramCredentialsResponse0;
impl ApiCall for AlterUserScramCredentialsRequest {
    type Response = AlterUserScramCredentialsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterUserScramCredentials
    }
    fn get_first_error(response: &AlterUserScramCredentialsResponse) -> Option<ApiError> {
        AlterUserScramCredentialsResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                AlterUserScramCredentialsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                AlterUserScramCredentialsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(
        version: i16,
        buf: &mut Bytes,
    ) -> (i32, AlterUserScramCredentialsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => AlterUserScramCredentialsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
            _ => AlterUserScramCredentialsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterUserScramCredentialsRequest0 {
    pub deletions: Vec<AlterUserScramCredentialsRequestDeletions0>,
    pub upsertions: Vec<AlterUserScramCredentialsRequestUpsertions0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterUserScramCredentialsRequestDeletions0 {
    pub name: String,
    pub mechanism: Int8,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterUserScramCredentialsRequestUpsertions0 {
    pub name: String,
    pub mechanism: Int8,
    pub iterations: Int32,
    pub salt: KafkaBytes,
    pub salted_password: KafkaBytes,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterUserScramCredentialsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AlterUserScramCredentialsResponseResults0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterUserScramCredentialsResponseResults0 {
    pub user: String,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub tag_buffer: TagBuffer,
}

impl AlterUserScramCredentialsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AlterUserScramCredentialsResponseResults0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
