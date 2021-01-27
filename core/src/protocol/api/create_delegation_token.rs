use super::prelude::*;

pub type CreateDelegationTokenRequest = CreateDelegationTokenRequest2;
pub type CreateDelegationTokenResponse = CreateDelegationTokenResponse2;
pub fn serialize_create_delegation_token_request(
    data: CreateDelegationTokenRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&CreateDelegationTokenRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&CreateDelegationTokenRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_create_delegation_token_response(
    version: i32,
    buf: &mut Bytes,
) -> CreateDelegationTokenResponse {
    match version {
        0 => CreateDelegationTokenResponse0::deserialize(buf).into(),
        1 => CreateDelegationTokenResponse1::deserialize(buf).into(),
        2 => CreateDelegationTokenResponse::deserialize(buf),
        _ => CreateDelegationTokenResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateDelegationTokenRequest0 {
    pub renewers: Vec<CreateDelegationTokenRequestRenewers0>,
    pub max_lifetime_ms: Int64,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateDelegationTokenRequestRenewers0 {
    pub principal_type: String,
    pub principal_name: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateDelegationTokenRequest1 {
    pub renewers: Vec<CreateDelegationTokenRequestRenewers1>,
    pub max_lifetime_ms: Int64,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateDelegationTokenRequestRenewers1 {
    pub principal_type: String,
    pub principal_name: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateDelegationTokenRequest2 {
    pub renewers: Vec<CreateDelegationTokenRequestRenewers2>,
    pub max_lifetime_ms: Int64,
}

#[derive(Default, Debug, ToBytes)]
pub struct CreateDelegationTokenRequestRenewers2 {
    pub principal_type: CompactString,
    pub principal_name: CompactString,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateDelegationTokenResponse0 {
    pub error_code: Int16,
    pub principal_type: String,
    pub principal_name: String,
    pub issue_timestamp_ms: Int64,
    pub expiry_timestamp_ms: Int64,
    pub max_timestamp_ms: Int64,
    pub token_id: String,
    pub hmac: KafkaBytes,
    pub throttle_time_ms: Int32,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateDelegationTokenResponse1 {
    pub error_code: Int16,
    pub principal_type: String,
    pub principal_name: String,
    pub issue_timestamp_ms: Int64,
    pub expiry_timestamp_ms: Int64,
    pub max_timestamp_ms: Int64,
    pub token_id: String,
    pub hmac: KafkaBytes,
    pub throttle_time_ms: Int32,
}

#[derive(Default, Debug, FromBytes)]
pub struct CreateDelegationTokenResponse2 {
    pub error_code: Int16,
    pub principal_type: CompactString,
    pub principal_name: CompactString,
    pub issue_timestamp_ms: Int64,
    pub expiry_timestamp_ms: Int64,
    pub max_timestamp_ms: Int64,
    pub token_id: CompactString,
    pub hmac: CompactBytes,
    pub throttle_time_ms: Int32,
}

impl TryFrom<CreateDelegationTokenRequest2> for CreateDelegationTokenRequest0 {
    type Error = Error;
    fn try_from(latest: CreateDelegationTokenRequest2) -> Result<Self, Self::Error> {
        Ok(CreateDelegationTokenRequest0 {
            renewers: latest
                .renewers
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            max_lifetime_ms: latest.max_lifetime_ms,
        })
    }
}

impl TryFrom<CreateDelegationTokenRequestRenewers2> for CreateDelegationTokenRequestRenewers0 {
    type Error = Error;
    fn try_from(latest: CreateDelegationTokenRequestRenewers2) -> Result<Self, Self::Error> {
        Ok(CreateDelegationTokenRequestRenewers0 {
            principal_type: latest.principal_type.into(),
            principal_name: latest.principal_name.into(),
        })
    }
}

impl TryFrom<CreateDelegationTokenRequest2> for CreateDelegationTokenRequest1 {
    type Error = Error;
    fn try_from(latest: CreateDelegationTokenRequest2) -> Result<Self, Self::Error> {
        Ok(CreateDelegationTokenRequest1 {
            renewers: latest
                .renewers
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            max_lifetime_ms: latest.max_lifetime_ms,
        })
    }
}

impl TryFrom<CreateDelegationTokenRequestRenewers2> for CreateDelegationTokenRequestRenewers1 {
    type Error = Error;
    fn try_from(latest: CreateDelegationTokenRequestRenewers2) -> Result<Self, Self::Error> {
        Ok(CreateDelegationTokenRequestRenewers1 {
            principal_type: latest.principal_type.into(),
            principal_name: latest.principal_name.into(),
        })
    }
}

impl From<CreateDelegationTokenResponse0> for CreateDelegationTokenResponse2 {
    fn from(older: CreateDelegationTokenResponse0) -> Self {
        CreateDelegationTokenResponse2 {
            error_code: older.error_code,
            principal_type: older.principal_type.into(),
            principal_name: older.principal_name.into(),
            issue_timestamp_ms: older.issue_timestamp_ms,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            max_timestamp_ms: older.max_timestamp_ms,
            token_id: older.token_id.into(),
            hmac: older.hmac.into(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<CreateDelegationTokenResponse1> for CreateDelegationTokenResponse2 {
    fn from(older: CreateDelegationTokenResponse1) -> Self {
        CreateDelegationTokenResponse2 {
            error_code: older.error_code,
            principal_type: older.principal_type.into(),
            principal_name: older.principal_name.into(),
            issue_timestamp_ms: older.issue_timestamp_ms,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            max_timestamp_ms: older.max_timestamp_ms,
            token_id: older.token_id.into(),
            hmac: older.hmac.into(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}
