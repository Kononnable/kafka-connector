use super::prelude::*;

pub type RenewDelegationTokenRequest = RenewDelegationTokenRequest2;
pub type RenewDelegationTokenResponse = RenewDelegationTokenResponse2;
pub fn serialize_renew_delegation_token_request(
    data: RenewDelegationTokenRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&RenewDelegationTokenRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&RenewDelegationTokenRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_renew_delegation_token_response(
    version: i32,
    buf: &mut Bytes,
) -> RenewDelegationTokenResponse {
    match version {
        0 => RenewDelegationTokenResponse0::deserialize(buf).into(),
        1 => RenewDelegationTokenResponse1::deserialize(buf).into(),
        2 => RenewDelegationTokenResponse::deserialize(buf),
        _ => RenewDelegationTokenResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct RenewDelegationTokenRequest0 {
    pub hmac: KafkaBytes,
    pub renew_period_ms: Int64,
}

#[derive(Default, ToBytes)]
pub struct RenewDelegationTokenRequest1 {
    pub hmac: KafkaBytes,
    pub renew_period_ms: Int64,
}

#[derive(Default, ToBytes)]
pub struct RenewDelegationTokenRequest2 {
    pub hmac: CompactBytes,
    pub renew_period_ms: Int64,
}

#[derive(Default, FromBytes)]
pub struct RenewDelegationTokenResponse0 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
}

#[derive(Default, FromBytes)]
pub struct RenewDelegationTokenResponse1 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
}

#[derive(Default, FromBytes)]
pub struct RenewDelegationTokenResponse2 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
}

impl TryFrom<RenewDelegationTokenRequest2> for RenewDelegationTokenRequest0 {
    type Error = Error;
    fn try_from(latest: RenewDelegationTokenRequest2) -> Result<Self, Self::Error> {
        Ok(RenewDelegationTokenRequest0 {
            hmac: latest.hmac.into(),
            renew_period_ms: latest.renew_period_ms,
        })
    }
}

impl TryFrom<RenewDelegationTokenRequest2> for RenewDelegationTokenRequest1 {
    type Error = Error;
    fn try_from(latest: RenewDelegationTokenRequest2) -> Result<Self, Self::Error> {
        Ok(RenewDelegationTokenRequest1 {
            hmac: latest.hmac.into(),
            renew_period_ms: latest.renew_period_ms,
        })
    }
}

impl From<RenewDelegationTokenResponse0> for RenewDelegationTokenResponse2 {
    fn from(older: RenewDelegationTokenResponse0) -> Self {
        RenewDelegationTokenResponse2 {
            error_code: older.error_code,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<RenewDelegationTokenResponse1> for RenewDelegationTokenResponse2 {
    fn from(older: RenewDelegationTokenResponse1) -> Self {
        RenewDelegationTokenResponse2 {
            error_code: older.error_code,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}
