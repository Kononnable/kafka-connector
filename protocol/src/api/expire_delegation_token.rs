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
                ExpireDelegationTokenRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                ExpireDelegationTokenRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &ExpireDelegationTokenRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &ExpireDelegationTokenRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, ExpireDelegationTokenResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => {
                ExpireDelegationTokenResponse0::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            1 => {
                ExpireDelegationTokenResponse1::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            2 => {
                ExpireDelegationTokenResponse::deserialize(buf, Self::is_flexible_version(version))
            }
            _ => {
                ExpireDelegationTokenResponse::deserialize(buf, Self::is_flexible_version(version))
            }
        };
        (correlation, response)
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
    pub hmac: KafkaBytes,
    pub expiry_time_period_ms: Int64,
    pub tag_buffer: TagBuffer,
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
    pub tag_buffer: Option<TagBuffer>,
}

impl From<ExpireDelegationTokenRequest2> for ExpireDelegationTokenRequest0 {
    fn from(latest: ExpireDelegationTokenRequest2) -> ExpireDelegationTokenRequest0 {
        ExpireDelegationTokenRequest0 {
            hmac: latest.hmac,
            expiry_time_period_ms: latest.expiry_time_period_ms,
        }
    }
}

impl From<ExpireDelegationTokenRequest2> for ExpireDelegationTokenRequest1 {
    fn from(latest: ExpireDelegationTokenRequest2) -> ExpireDelegationTokenRequest1 {
        ExpireDelegationTokenRequest1 {
            hmac: latest.hmac,
            expiry_time_period_ms: latest.expiry_time_period_ms,
        }
    }
}

impl From<ExpireDelegationTokenResponse0> for ExpireDelegationTokenResponse2 {
    fn from(older: ExpireDelegationTokenResponse0) -> Self {
        ExpireDelegationTokenResponse2 {
            error_code: older.error_code,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            throttle_time_ms: older.throttle_time_ms,
            ..ExpireDelegationTokenResponse2::default()
        }
    }
}

impl From<ExpireDelegationTokenResponse1> for ExpireDelegationTokenResponse2 {
    fn from(older: ExpireDelegationTokenResponse1) -> Self {
        ExpireDelegationTokenResponse2 {
            error_code: older.error_code,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            throttle_time_ms: older.throttle_time_ms,
            ..ExpireDelegationTokenResponse2::default()
        }
    }
}
