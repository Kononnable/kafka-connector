use super::prelude::*;

pub type AlterUserScramCredentialsRequest = AlterUserScramCredentialsRequest0;
pub type AlterUserScramCredentialsResponse = AlterUserScramCredentialsResponse0;
pub fn serialize_alter_user_scram_credentials_request(
    data: AlterUserScramCredentialsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_alter_user_scram_credentials_response<T>(
    version: i32,
    buf: &mut T,
) -> AlterUserScramCredentialsResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        1 => AlterUserScramCredentialsResponse::deserialize(buf),
        _ => AlterUserScramCredentialsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct AlterUserScramCredentialsRequest0 {
    pub deletions: Vec<AlterUserScramCredentialsRequestDeletions0>,
    pub upsertions: Vec<AlterUserScramCredentialsRequestUpsertions0>,
}

#[derive(Default, ToBytes)]
pub struct AlterUserScramCredentialsRequestDeletions0 {
    pub name: CompactString,
    pub mechanism: Int8,
}

#[derive(Default, ToBytes)]
pub struct AlterUserScramCredentialsRequestUpsertions0 {
    pub name: CompactString,
    pub mechanism: Int8,
    pub iterations: Int32,
    pub salt: CompactBytes,
    pub salted_password: CompactBytes,
}

#[derive(Default, FromBytes)]
pub struct AlterUserScramCredentialsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AlterUserScramCredentialsResponseResults0>,
}

#[derive(Default, FromBytes)]
pub struct AlterUserScramCredentialsResponseResults0 {
    pub user: CompactString,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
}
