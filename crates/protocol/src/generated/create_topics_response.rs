use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreateTopicsResponse {
    pub throttle_time_ms: i32,
    pub topics: Vec<CreatableTopicResult>,
}

#[derive(Debug, Default, Clone)]
pub struct CreatableTopicResult {
    pub name: String,
    pub error_code: i16,
    pub error_message: String,
}

impl ApiResponse for CreateTopicsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 2 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = if version >= 0 {
            Vec::<CreatableTopicResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            CreateTopicsResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl FromBytes for CreatableTopicResult {
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
        let error_message = if version >= 1 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        CreatableTopicResult {
            name,
            error_code,
            error_message,
        }
    }
}
