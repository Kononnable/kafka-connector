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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> AlterUserScramCredentialsResponse {
        match version {
            0 => AlterUserScramCredentialsResponse::deserialize(buf),
            _ => AlterUserScramCredentialsResponse::deserialize(buf),
        }
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
    pub name: CompactString,
    pub mechanism: Int8,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterUserScramCredentialsRequestUpsertions0 {
    pub name: CompactString,
    pub mechanism: Int8,
    pub iterations: Int32,
    pub salt: CompactBytes,
    pub salted_password: CompactBytes,
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
    pub user: CompactString,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub tag_buffer: TagBuffer,
}
