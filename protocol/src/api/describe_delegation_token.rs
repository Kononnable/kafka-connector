use super::prelude::*;

pub type DescribeDelegationTokenRequest = DescribeDelegationTokenRequest2;
pub type DescribeDelegationTokenResponse = DescribeDelegationTokenResponse2;
impl ApiCall for DescribeDelegationTokenRequest {
    type Response = DescribeDelegationTokenResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeDelegationToken
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&DescribeDelegationTokenRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&DescribeDelegationTokenRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> DescribeDelegationTokenResponse {
        match version {
            0 => DescribeDelegationTokenResponse0::deserialize(buf).into(),
            1 => DescribeDelegationTokenResponse1::deserialize(buf).into(),
            2 => DescribeDelegationTokenResponse::deserialize(buf),
            _ => DescribeDelegationTokenResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeDelegationTokenRequest0 {
    pub owners: Vec<DescribeDelegationTokenRequestOwners0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeDelegationTokenRequestOwners0 {
    pub principal_type: String,
    pub principal_name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeDelegationTokenRequest1 {
    pub owners: Vec<DescribeDelegationTokenRequestOwners1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeDelegationTokenRequestOwners1 {
    pub principal_type: String,
    pub principal_name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeDelegationTokenRequest2 {
    pub owners: Vec<DescribeDelegationTokenRequestOwners2>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeDelegationTokenRequestOwners2 {
    pub principal_type: CompactString,
    pub principal_name: CompactString,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponse0 {
    pub error_code: Int16,
    pub tokens: Vec<DescribeDelegationTokenResponseTokens0>,
    pub throttle_time_ms: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponseTokens0 {
    pub principal_type: String,
    pub principal_name: String,
    pub issue_timestamp: Int64,
    pub expiry_timestamp: Int64,
    pub max_timestamp: Int64,
    pub token_id: String,
    pub hmac: KafkaBytes,
    pub renewers: Vec<DescribeDelegationTokenResponseTokensRenewers0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponseTokensRenewers0 {
    pub principal_type: String,
    pub principal_name: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponse1 {
    pub error_code: Int16,
    pub tokens: Vec<DescribeDelegationTokenResponseTokens1>,
    pub throttle_time_ms: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponseTokens1 {
    pub principal_type: String,
    pub principal_name: String,
    pub issue_timestamp: Int64,
    pub expiry_timestamp: Int64,
    pub max_timestamp: Int64,
    pub token_id: String,
    pub hmac: KafkaBytes,
    pub renewers: Vec<DescribeDelegationTokenResponseTokensRenewers1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponseTokensRenewers1 {
    pub principal_type: String,
    pub principal_name: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponse2 {
    pub error_code: Int16,
    pub tokens: Vec<DescribeDelegationTokenResponseTokens2>,
    pub throttle_time_ms: Int32,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponseTokens2 {
    pub principal_type: CompactString,
    pub principal_name: CompactString,
    pub issue_timestamp: Int64,
    pub expiry_timestamp: Int64,
    pub max_timestamp: Int64,
    pub token_id: CompactString,
    pub hmac: CompactBytes,
    pub renewers: Vec<DescribeDelegationTokenResponseTokensRenewers2>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponseTokensRenewers2 {
    pub principal_type: CompactString,
    pub principal_name: CompactString,
    pub tag_buffer: Optional<TagBuffer>,
}

impl TryFrom<DescribeDelegationTokenRequest2> for DescribeDelegationTokenRequest0 {
    type Error = Error;
    fn try_from(latest: DescribeDelegationTokenRequest2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeDelegationTokenRequest",
                0,
                "tag_buffer",
            ));
        }
        Ok(DescribeDelegationTokenRequest0 {
            owners: latest
                .owners
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DescribeDelegationTokenRequestOwners2> for DescribeDelegationTokenRequestOwners0 {
    type Error = Error;
    fn try_from(latest: DescribeDelegationTokenRequestOwners2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeDelegationTokenRequestOwners",
                0,
                "tag_buffer",
            ));
        }
        Ok(DescribeDelegationTokenRequestOwners0 {
            principal_type: latest.principal_type.into(),
            principal_name: latest.principal_name.into(),
        })
    }
}

impl TryFrom<DescribeDelegationTokenRequest2> for DescribeDelegationTokenRequest1 {
    type Error = Error;
    fn try_from(latest: DescribeDelegationTokenRequest2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeDelegationTokenRequest",
                1,
                "tag_buffer",
            ));
        }
        Ok(DescribeDelegationTokenRequest1 {
            owners: latest
                .owners
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DescribeDelegationTokenRequestOwners2> for DescribeDelegationTokenRequestOwners1 {
    type Error = Error;
    fn try_from(latest: DescribeDelegationTokenRequestOwners2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "DescribeDelegationTokenRequestOwners",
                1,
                "tag_buffer",
            ));
        }
        Ok(DescribeDelegationTokenRequestOwners1 {
            principal_type: latest.principal_type.into(),
            principal_name: latest.principal_name.into(),
        })
    }
}

impl From<DescribeDelegationTokenResponse0> for DescribeDelegationTokenResponse2 {
    fn from(older: DescribeDelegationTokenResponse0) -> Self {
        DescribeDelegationTokenResponse2 {
            error_code: older.error_code,
            tokens: older.tokens.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
            ..DescribeDelegationTokenResponse2::default()
        }
    }
}

impl From<DescribeDelegationTokenResponseTokens0> for DescribeDelegationTokenResponseTokens2 {
    fn from(older: DescribeDelegationTokenResponseTokens0) -> Self {
        DescribeDelegationTokenResponseTokens2 {
            principal_type: older.principal_type.into(),
            principal_name: older.principal_name.into(),
            issue_timestamp: older.issue_timestamp,
            expiry_timestamp: older.expiry_timestamp,
            max_timestamp: older.max_timestamp,
            token_id: older.token_id.into(),
            hmac: older.hmac.into(),
            renewers: older.renewers.into_iter().map(|el| el.into()).collect(),
            ..DescribeDelegationTokenResponseTokens2::default()
        }
    }
}

impl From<DescribeDelegationTokenResponseTokensRenewers0>
    for DescribeDelegationTokenResponseTokensRenewers2
{
    fn from(older: DescribeDelegationTokenResponseTokensRenewers0) -> Self {
        DescribeDelegationTokenResponseTokensRenewers2 {
            principal_type: older.principal_type.into(),
            principal_name: older.principal_name.into(),
            ..DescribeDelegationTokenResponseTokensRenewers2::default()
        }
    }
}

impl From<DescribeDelegationTokenResponse1> for DescribeDelegationTokenResponse2 {
    fn from(older: DescribeDelegationTokenResponse1) -> Self {
        DescribeDelegationTokenResponse2 {
            error_code: older.error_code,
            tokens: older.tokens.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
            ..DescribeDelegationTokenResponse2::default()
        }
    }
}

impl From<DescribeDelegationTokenResponseTokens1> for DescribeDelegationTokenResponseTokens2 {
    fn from(older: DescribeDelegationTokenResponseTokens1) -> Self {
        DescribeDelegationTokenResponseTokens2 {
            principal_type: older.principal_type.into(),
            principal_name: older.principal_name.into(),
            issue_timestamp: older.issue_timestamp,
            expiry_timestamp: older.expiry_timestamp,
            max_timestamp: older.max_timestamp,
            token_id: older.token_id.into(),
            hmac: older.hmac.into(),
            renewers: older.renewers.into_iter().map(|el| el.into()).collect(),
            ..DescribeDelegationTokenResponseTokens2::default()
        }
    }
}

impl From<DescribeDelegationTokenResponseTokensRenewers1>
    for DescribeDelegationTokenResponseTokensRenewers2
{
    fn from(older: DescribeDelegationTokenResponseTokensRenewers1) -> Self {
        DescribeDelegationTokenResponseTokensRenewers2 {
            principal_type: older.principal_type.into(),
            principal_name: older.principal_name.into(),
            ..DescribeDelegationTokenResponseTokensRenewers2::default()
        }
    }
}
