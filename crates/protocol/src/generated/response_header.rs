use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct ResponseHeader {
    /// The correlation ID of this response.
    pub correlation_id: i32,
}

impl ResponseHeader {
    pub fn deserialize(version: i16, bytes: &mut Bytes) -> ResponseHeader {
        let correlation_id = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ResponseHeader { correlation_id }
    }
}

impl Default for ResponseHeader {
    fn default() -> Self {
        Self {
            correlation_id: Default::default(),
        }
    }
}
