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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> DescribeUserScramCredentialsResponse {
        match version {
            0 => DescribeUserScramCredentialsResponse::deserialize(buf),
            _ => DescribeUserScramCredentialsResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeUserScramCredentialsRequest0 {
    pub users: Vec<DescribeUserScramCredentialsRequestUsers0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeUserScramCredentialsRequestUsers0 {
    pub name: CompactString,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeUserScramCredentialsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub results: Vec<DescribeUserScramCredentialsResponseResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeUserScramCredentialsResponseResults0 {
    pub user: CompactString,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub credential_infos: Vec<DescribeUserScramCredentialsResponseResultsCredentialInfos0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeUserScramCredentialsResponseResultsCredentialInfos0 {
    pub mechanism: Int8,
    pub iterations: Int32,
}
