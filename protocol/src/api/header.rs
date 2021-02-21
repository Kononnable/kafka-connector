use std::fmt::Debug;

use super::prelude::*;

#[derive(Debug, ToBytes)]
pub struct HeaderRequest<'a> {
    pub api_key: Int16,
    pub api_version: Int16,
    pub correlation_id: Int32,
    pub client_id: NullableStr<'a>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}

impl<'a> HeaderRequest<'a> {
    pub fn new(
        api_key: ApiNumbers,
        api_version: u16,
        correlation_id: Int32,
        client_id: &str,
    ) -> HeaderRequest {
        HeaderRequest {
            api_key: api_key as i16,
            api_version: api_version as Int16,
            correlation_id,
            client_id: NullableStr::Some(client_id),
            tag_buffer: TagBuffer {},
        }
    }
}

#[derive(Debug, FromBytes)]
pub struct HeaderResponse {
    pub correlation: Int32,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}

#[derive(Debug, FromBytes)]
pub struct OwnedHeaderRequest {
    pub api_key: Int16,
    pub api_version: Int16,
    pub correlation_id: Int32,
    pub client_id: NullableString,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
