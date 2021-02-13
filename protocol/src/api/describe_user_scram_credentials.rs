use super::prelude::*;

pub type DescribeUserScramCredentialsRequest = DescribeUserScramCredentialsRequest0;
pub type DescribeUserScramCredentialsResponse = DescribeUserScramCredentialsResponse0;
impl ApiCall for DescribeUserScramCredentialsRequest {
    type Response = DescribeUserScramCredentialsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeUserScramCredentials
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
                DescribeUserScramCredentialsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DescribeUserScramCredentialsRequest::get_api_key(),
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
    ) -> (i32, DescribeUserScramCredentialsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => DescribeUserScramCredentialsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
            _ => DescribeUserScramCredentialsResponse::deserialize(
                buf,
                Self::is_flexible_version(version),
            ),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeUserScramCredentialsRequest0 {
    pub users: Vec<DescribeUserScramCredentialsRequestUsers0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeUserScramCredentialsRequestUsers0 {
    pub name: String,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeUserScramCredentialsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub results: Vec<DescribeUserScramCredentialsResponseResults0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeUserScramCredentialsResponseResults0 {
    pub user: String,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub credential_infos: Vec<DescribeUserScramCredentialsResponseResultsCredentialInfos0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeUserScramCredentialsResponseResultsCredentialInfos0 {
    pub mechanism: Int8,
    pub iterations: Int32,
    pub tag_buffer: TagBuffer,
}
