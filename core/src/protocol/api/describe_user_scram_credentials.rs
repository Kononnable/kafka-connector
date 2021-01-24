use super::prelude::*;

pub type DescribeUserScramCredentialsRequest = DescribeUserScramCredentialsRequest0;
pub type DescribeUserScramCredentialsResponse = DescribeUserScramCredentialsResponse0;
pub fn serialize_describe_user_scram_credentials_request(
    data: DescribeUserScramCredentialsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_describe_user_scram_credentials_response<T>(
    version: i32,
    buf: &mut T,
) -> DescribeUserScramCredentialsResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        1 => DescribeUserScramCredentialsResponse::deserialize(buf),
        _ => DescribeUserScramCredentialsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct DescribeUserScramCredentialsRequest0 {
    pub users: Vec<DescribeUserScramCredentialsRequestUsers0>,
}

#[derive(Default, ToBytes)]
pub struct DescribeUserScramCredentialsRequestUsers0 {
    pub name: CompactString,
}

#[derive(Default, FromBytes)]
pub struct DescribeUserScramCredentialsResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub results: Vec<DescribeUserScramCredentialsResponseResults0>,
}

#[derive(Default, FromBytes)]
pub struct DescribeUserScramCredentialsResponseResults0 {
    pub user: CompactString,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub credential_infos: Vec<DescribeUserScramCredentialsResponseResultsCredentialInfos0>,
}

#[derive(Default, FromBytes)]
pub struct DescribeUserScramCredentialsResponseResultsCredentialInfos0 {
    pub mechanism: Int8,
    pub iterations: Int32,
}
