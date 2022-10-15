use std::fmt::Debug;

use super::prelude::*;

#[derive(Debug)]
pub struct HeaderRequest<'a> {
    pub api_key: Int16,
    pub api_version: Int16,
    pub correlation_id: Int32,
    pub client_id: NullableStr<'a>,
    // #[min_version = 2]
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
    pub fn serialize(&self, bytes: &mut BytesMut, is_flexible: bool, version: u16) {
        self.api_key.serialize(bytes, is_flexible);
        self.api_version.serialize(bytes, is_flexible);
        self.correlation_id.serialize(bytes, is_flexible);
        self.client_id.serialize(bytes, is_flexible);
        if version >= 2 {
            self.tag_buffer.serialize(bytes, is_flexible);
        }
    }
}

#[derive(Debug)]
pub struct HeaderResponse {
    pub correlation: Int32,
    // #[min_version = 2]
    pub tag_buffer: TagBuffer,
}

impl HeaderResponse {
    pub fn deserialize(buf: &mut Bytes, is_flexible_version: bool, version: u16) -> Self {
        let correlation = Int32::deserialize(buf, is_flexible_version, version);
        let tag_buffer = if version >= 2 {
            TagBuffer::deserialize(buf, is_flexible_version, version)
        } else {
            TagBuffer::default()
        };
        HeaderResponse {
            correlation,
            tag_buffer,
        }
    }
}
