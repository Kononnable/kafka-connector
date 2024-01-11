use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct CreateTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Results for each topic we tried to create.
    pub topics: Vec<CreatableTopicResult>,
}

#[derive(Debug, Clone)]
pub struct CreatableTopicResult {
    /// The topic name.
    pub name: String,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The error message, or null if there was no error.
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

impl Default for CreateTopicsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            topics: Default::default(),
        }
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

impl Default for CreatableTopicResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
