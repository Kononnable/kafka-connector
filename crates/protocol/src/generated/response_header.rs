use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResponseHeader {
    /// The correlation ID of this response.
    pub correlation_id: i32,
}

impl ResponseHeader {
    pub fn deserialize(version: i16, bytes: &mut BytesMut) -> ResponseHeader {
        let correlation_id = i32::deserialize(version, bytes);
        ResponseHeader { correlation_id }
    }
}
