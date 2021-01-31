use super::prelude::*;

pub type CreateDelegationTokenRequest = CreateDelegationTokenRequest2;
pub type CreateDelegationTokenResponse = CreateDelegationTokenResponse2;
impl ApiCall for CreateDelegationTokenRequest {
    type Response = CreateDelegationTokenResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::CreateDelegationToken
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            _ => true,
        }
    }
    fn serialize(
        self,
        version: i16,
        buf: &mut BytesMut,
        correlation_id: i32,
        client_id: &str,
    ) -> Result<(), Error> {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                CreateDelegationTokenRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                CreateDelegationTokenRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &CreateDelegationTokenRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &CreateDelegationTokenRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, CreateDelegationTokenResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => {
                CreateDelegationTokenResponse0::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            1 => {
                CreateDelegationTokenResponse1::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            2 => {
                CreateDelegationTokenResponse::deserialize(buf, Self::is_flexible_version(version))
            }
            _ => {
                CreateDelegationTokenResponse::deserialize(buf, Self::is_flexible_version(version))
            }
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateDelegationTokenRequest0 {
    pub renewers: Vec<CreateDelegationTokenRequestRenewers0>,
    pub max_lifetime_ms: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateDelegationTokenRequestRenewers0 {
    pub principal_type: String,
    pub principal_name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateDelegationTokenRequest1 {
    pub renewers: Vec<CreateDelegationTokenRequestRenewers1>,
    pub max_lifetime_ms: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateDelegationTokenRequestRenewers1 {
    pub principal_type: String,
    pub principal_name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateDelegationTokenRequest2 {
    pub renewers: Vec<CreateDelegationTokenRequestRenewers2>,
    pub max_lifetime_ms: Int64,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateDelegationTokenRequestRenewers2 {
    pub principal_type: String,
    pub principal_name: String,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
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

#[derive(Default, Debug, Clone, FromBytes)]
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

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateDelegationTokenResponse2 {
    pub error_code: Int16,
    pub principal_type: String,
    pub principal_name: String,
    pub issue_timestamp_ms: Int64,
    pub expiry_timestamp_ms: Int64,
    pub max_timestamp_ms: Int64,
    pub token_id: String,
    pub hmac: KafkaBytes,
    pub throttle_time_ms: Int32,
    pub tag_buffer: Optional<TagBuffer>,
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
            principal_type: latest.principal_type,
            principal_name: latest.principal_name,
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
            principal_type: latest.principal_type,
            principal_name: latest.principal_name,
        })
    }
}

impl From<CreateDelegationTokenResponse0> for CreateDelegationTokenResponse2 {
    fn from(older: CreateDelegationTokenResponse0) -> Self {
        CreateDelegationTokenResponse2 {
            error_code: older.error_code,
            principal_type: older.principal_type,
            principal_name: older.principal_name,
            issue_timestamp_ms: older.issue_timestamp_ms,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            max_timestamp_ms: older.max_timestamp_ms,
            token_id: older.token_id,
            hmac: older.hmac,
            throttle_time_ms: older.throttle_time_ms,
            ..CreateDelegationTokenResponse2::default()
        }
    }
}

impl From<CreateDelegationTokenResponse1> for CreateDelegationTokenResponse2 {
    fn from(older: CreateDelegationTokenResponse1) -> Self {
        CreateDelegationTokenResponse2 {
            error_code: older.error_code,
            principal_type: older.principal_type,
            principal_name: older.principal_name,
            issue_timestamp_ms: older.issue_timestamp_ms,
            expiry_timestamp_ms: older.expiry_timestamp_ms,
            max_timestamp_ms: older.max_timestamp_ms,
            token_id: older.token_id,
            hmac: older.hmac,
            throttle_time_ms: older.throttle_time_ms,
            ..CreateDelegationTokenResponse2::default()
        }
    }
}
