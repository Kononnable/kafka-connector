use super::prelude::*;

pub type RenewDelegationTokenRequest = RenewDelegationTokenRequest2;
pub type RenewDelegationTokenResponse = RenewDelegationTokenResponse2;
impl ApiCall for RenewDelegationTokenRequest {
    type Response = RenewDelegationTokenResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::RenewDelegationToken
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&RenewDelegationTokenRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&RenewDelegationTokenRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> RenewDelegationTokenResponse {
        match version {
            0 => RenewDelegationTokenResponse0::deserialize(buf).into(),
            1 => RenewDelegationTokenResponse1::deserialize(buf).into(),
            2 => RenewDelegationTokenResponse::deserialize(buf),
            _ => RenewDelegationTokenResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct RenewDelegationTokenRequest0 {
    pub hmac: KafkaBytes,
    pub renew_period_ms: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct RenewDelegationTokenRequest1 {
    pub hmac: KafkaBytes,
    pub renew_period_ms: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct RenewDelegationTokenRequest2 {
    pub hmac: CompactBytes,
    pub renew_period_ms: Int64,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct RenewDelegationTokenResponse0 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct RenewDelegationTokenResponse1 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct RenewDelegationTokenResponse2 {
    pub error_code: Int16,
    pub expiry_timestamp_ms: Int64,
    pub throttle_time_ms: Int32,
    pub tag_buffer: Optional<TagBuffer>,
}

impl TryFrom<RenewDelegationTokenRequest2> for RenewDelegationTokenRequest0 {
    type Error = Error;
    fn try_from(latest: RenewDelegationTokenRequest2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "RenewDelegationTokenRequest",
                0,
                "tag_buffer",
            ));
        }
        Ok(RenewDelegationTokenRequest0 {
            hmac: latest.hmac.into(),
            renew_period_ms: latest.renew_period_ms,
        })
    }
}

impl TryFrom<RenewDelegationTokenRequest2> for RenewDelegationTokenRequest1 {
    type Error = Error;
    fn try_from(latest: RenewDelegationTokenRequest2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "RenewDelegationTokenRequest",
                1,
                "tag_buffer",
            ));
        }
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
            ..RenewDelegationTokenResponse2::default()
        }
    }
}

impl From<RenewDelegationTokenResponse1> for RenewDelegationTokenResponse2 {
    fn from(older: RenewDelegationTokenResponse1) -> Self {
        RenewDelegationTokenResponse2 {
            error_code: older.error_code,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            throttle_time_ms: older.throttle_time_ms,
            ..RenewDelegationTokenResponse2::default()
        }
    }
}
