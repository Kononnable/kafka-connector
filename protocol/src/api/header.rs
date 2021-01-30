use std::fmt::Debug;

use super::prelude::*;

#[derive(Debug, FromBytes)]
pub struct KafkaResponse<R>
where
    R: FromBytes + Debug,
{
    pub header: HeaderResponse,
    pub response: R,
}

#[derive(Debug, ToBytes)]
pub struct KafkaRequest<R>
where
    R: ToBytes + Debug,
{
    pub header: HeaderRequest,
    pub response: R,
}

#[derive(Debug, ToBytes)]
pub struct HeaderRequest {
    pub api_key: Int16,
    pub api_version: Int16,
    pub correlation_id: Int32,
    pub client_id: Option<String>,
    pub tag_buffer: TagBuffer,
}

impl HeaderRequest {
    pub fn new(
        api_key: Int16,
        api_version: Int16,
        correlation_id: Int32,
        client_id: String,
    ) -> HeaderRequest {
        HeaderRequest {
            api_key,
            api_version,
            correlation_id,
            client_id: Some(client_id),
            tag_buffer: TagBuffer {},
        }
    }
}

#[derive(Debug, FromBytes)]
pub struct HeaderResponse {
    pub correlation: Int32,
    pub tag_buffer: TagBuffer,
}
