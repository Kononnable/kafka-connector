use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ResponseHeader {
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
