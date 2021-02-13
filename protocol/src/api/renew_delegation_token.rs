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
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                RenewDelegationTokenRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                RenewDelegationTokenRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &RenewDelegationTokenRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &RenewDelegationTokenRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, RenewDelegationTokenResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => {
                RenewDelegationTokenResponse0::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            1 => {
                RenewDelegationTokenResponse1::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            2 => RenewDelegationTokenResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => RenewDelegationTokenResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
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
    pub hmac: KafkaBytes,
    pub renew_period_ms: Int64,
    pub tag_buffer: TagBuffer,
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
    pub tag_buffer: Option<TagBuffer>,
}

impl From<RenewDelegationTokenRequest2> for RenewDelegationTokenRequest0 {
    fn from(latest: RenewDelegationTokenRequest2) -> RenewDelegationTokenRequest0 {
        RenewDelegationTokenRequest0 {
            hmac: latest.hmac,
            renew_period_ms: latest.renew_period_ms,
        }
    }
}

impl From<RenewDelegationTokenRequest2> for RenewDelegationTokenRequest1 {
    fn from(latest: RenewDelegationTokenRequest2) -> RenewDelegationTokenRequest1 {
        RenewDelegationTokenRequest1 {
            hmac: latest.hmac,
            renew_period_ms: latest.renew_period_ms,
        }
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
