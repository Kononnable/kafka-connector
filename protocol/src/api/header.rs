use std::fmt::Debug;

use super::prelude::*;

#[derive(Debug, ToBytes)]
pub struct HeaderRequest1<'a> {
    pub api_key: Int16,
    pub api_version: Int16,
    pub correlation_id: Int32,
    pub client_id: Option<&'a str>,
}

impl<'a> HeaderRequest1<'a> {
    pub fn new(
        api_key: ApiNumbers,
        api_version: Int16,
        correlation_id: Int32,
        client_id: &str,
    ) -> HeaderRequest1 {
        HeaderRequest1 {
            api_key: api_key as i16,
            api_version,
            correlation_id,
            client_id: Some(client_id),
        }
    }
}

#[derive(Debug, ToBytes)]
pub struct HeaderRequest2<'a> {
    pub api_key: Int16,
    pub api_version: Int16,
    pub correlation_id: Int32,
    pub client_id: Option<&'a str>,
    pub tag_buffer: TagBuffer,
}

impl<'a> HeaderRequest2<'a> {
    pub fn new(
        api_key: ApiNumbers,
        api_version: Int16,
        correlation_id: Int32,
        client_id: &str,
    ) -> HeaderRequest2 {
        HeaderRequest2 {
            api_key: api_key as i16,
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
}

#[derive(Debug, FromBytes)]
pub struct HeaderResponse2 {
    pub correlation: Int32,
    pub tag_buffer: TagBuffer,
}
