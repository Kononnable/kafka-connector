use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteTopicsResponse {
    pub throttle_time_ms: i32,
    pub responses: Vec<DeletableTopicResult>,
}

#[derive(Debug, Default, Clone)]
pub struct DeletableTopicResult {
    pub name: String,
    pub error_code: i16,
}

impl ApiResponse for DeleteTopicsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 1 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let responses = if version >= 0 {
            Vec::<DeletableTopicResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            DeleteTopicsResponse {
                throttle_time_ms,
                responses,
            },
        )
    }
}

impl FromBytes for DeletableTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DeletableTopicResult { name, error_code }
    }
}
