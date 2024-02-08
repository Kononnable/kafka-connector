use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreateTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Results for each topic we tried to create.
    pub topics: Vec<CreatableTopicResult>,
}

#[derive(Clone, Debug, Default)]
pub struct CreatableTopicResult {
    /// The topic name.
    pub name: String,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The error message, or null if there was no error.
    pub error_message: Option<String>,
}

impl ApiResponse for CreateTopicsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 2 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let topics = Vec::<CreatableTopicResult>::deserialize(version, bytes);
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
        let name = String::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let error_message = if version >= 1 {
            Option::<String>::deserialize(version, bytes)
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
