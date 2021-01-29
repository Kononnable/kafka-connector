use super::prelude::*;

pub type ExpireDelegationTokenRequest = ExpireDelegationTokenRequest2;
pub type ExpireDelegationTokenResponse = ExpireDelegationTokenResponse2;
impl ApiCall for ExpireDelegationTokenRequest {
    type Response = ExpireDelegationTokenResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ExpireDelegationToken
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&ExpireDelegationTokenRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&ExpireDelegationTokenRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> ExpireDelegationTokenResponse {
        match version {
            0 => ExpireDelegationTokenResponse0::deserialize(buf).into(),
            1 => ExpireDelegationTokenResponse1::deserialize(buf).into(),
            2 => ExpireDelegationTokenResponse::deserialize(buf),
            _ => ExpireDelegationTokenResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ExpireDelegationTokenRequest0 {
    pub hmac: KafkaBytes,
    pub expiry_time_period_ms: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ExpireDelegationTokenRequest1 {
    pub hmac: KafkaBytes,
    pub expiry_time_period_ms: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ExpireDelegationTokenRequest2 {
    pub hmac: CompactBytes,
    pub expiry_time_period_ms: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ExpireDelegationTokenResponse0 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ExpireDelegationTokenResponse1 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ExpireDelegationTokenResponse2 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
}

impl TryFrom<ExpireDelegationTokenRequest2> for ExpireDelegationTokenRequest0 {
    type Error = Error;
    fn try_from(latest: ExpireDelegationTokenRequest2) -> Result<Self, Self::Error> {
        Ok(ExpireDelegationTokenRequest0 {
            hmac: latest.hmac.into(),
            expiry_time_period_ms: latest.expiry_time_period_ms,
        })
    }
}

impl TryFrom<ExpireDelegationTokenRequest2> for ExpireDelegationTokenRequest1 {
    type Error = Error;
    fn try_from(latest: ExpireDelegationTokenRequest2) -> Result<Self, Self::Error> {
        Ok(ExpireDelegationTokenRequest1 {
            hmac: latest.hmac.into(),
            expiry_time_period_ms: latest.expiry_time_period_ms,
        })
    }
}

impl From<ExpireDelegationTokenResponse0> for ExpireDelegationTokenResponse2 {
    fn from(older: ExpireDelegationTokenResponse0) -> Self {
        ExpireDelegationTokenResponse2 {
            error_code: older.error_code,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ExpireDelegationTokenResponse1> for ExpireDelegationTokenResponse2 {
    fn from(older: ExpireDelegationTokenResponse1) -> Self {
        ExpireDelegationTokenResponse2 {
            error_code: older.error_code,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}
