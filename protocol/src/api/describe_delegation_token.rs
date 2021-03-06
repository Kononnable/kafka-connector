use super::prelude::*;
pub type DescribeDelegationTokenRequest = DescribeDelegationTokenRequest0;
impl ApiCall for DescribeDelegationTokenRequest0 {
    type Response = DescribeDelegationTokenResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeDelegationToken
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 2
    }
    fn serialize(&self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(self, buf, Self::is_flexible_version(version), version);
    }
    fn deserialize_response(version: u16, buf: &mut Bytes) -> (i32, Self::Response) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse::deserialize(buf, false, 2).correlation,
            false => HeaderResponse::deserialize(buf, false, 1).correlation,
        };
        let response =
            Self::Response::deserialize(buf, Self::is_flexible_version(version), version);
        (correlation, response)
    }
    fn deserialize_request(version: u16, buf: &mut Bytes) -> (OwnedHeaderRequest, Self) {
        let header = match Self::is_flexible_version(version) {
            true => OwnedHeaderRequest::deserialize(buf, false, 2),
            false => OwnedHeaderRequest::deserialize(buf, false, 1),
        };
        let request = Self::deserialize(buf, Self::is_flexible_version(version), version);
        (header, request)
    }
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct DescribeDelegationTokenRequest0 {
    #[min_version = 0]
    pub owners: Vec<DescribeDelegationTokenRequestOwners0>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct DescribeDelegationTokenRequestOwners0 {
    #[min_version = 0]
    pub principal_type: String,
    #[min_version = 0]
    pub principal_name: String,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponse0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub tokens: Vec<DescribeDelegationTokenResponseTokens0>,
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponseTokens0 {
    #[min_version = 0]
    pub principal_type: String,
    #[min_version = 0]
    pub principal_name: String,
    #[min_version = 0]
    pub issue_timestamp: Int64,
    #[min_version = 0]
    pub expiry_timestamp: Int64,
    #[min_version = 0]
    pub max_timestamp: Int64,
    #[min_version = 0]
    pub token_id: String,
    #[min_version = 0]
    pub hmac: KafkaBytes,
    #[min_version = 0]
    pub renewers: Vec<DescribeDelegationTokenResponseTokensRenewers0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeDelegationTokenResponseTokensRenewers0 {
    #[min_version = 0]
    pub principal_type: String,
    #[min_version = 0]
    pub principal_name: String,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}

impl DescribeDelegationTokenResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if self.error_code != 0 {
            return Some(self.error_code.into());
        }
        None
    }
}
