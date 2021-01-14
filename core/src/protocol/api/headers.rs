use crate::log;

use super::super::from_bytes::FromBytes;
use super::super::to_bytes::ToBytes;

use kafka_connector_derive::FromBytes;
use kafka_connector_derive::ToBytes;

#[derive(Debug, FromBytes)]
pub struct KafkaResponse<R>
where
    R: FromBytes,
{
    pub header: HeaderResponse,
    pub response: R,
}

#[derive(Debug, ToBytes)]
pub struct KafkaRequest<R>
where
    R: ToBytes,
{
    pub header: HeaderRequest,
    pub response: R,
}

#[derive(Debug, ToBytes)]
pub struct HeaderRequest {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: String,
}

impl HeaderRequest {
    pub fn new(
        api_key: i16,
        api_version: i16,
        correlation_id: i32,
        client_id: String,
    ) -> HeaderRequest {
        HeaderRequest {
            api_key: api_key,
            api_version: api_version,
            correlation_id: correlation_id,
            client_id: client_id,
        }
    }
}

#[derive(Debug, FromBytes)]
pub struct HeaderResponse {
    pub correlation: i32,
}
