use super::super::prelude::*;

/// Version 1 adds a per-topic error message string.
/// Version 2 adds the throttle time.
/// Starting in version 3, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreateTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Results for each topic we tried to create.
    pub topics: IndexMap<CreatableTopicResultKey, CreatableTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct CreatableTopicResultKey {
    /// The topic name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatableTopicResult {
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
        let topics =
            IndexMap::<CreatableTopicResultKey, CreatableTopicResult>::deserialize(version, bytes);
        (
            header,
            CreateTopicsResponse {
                throttle_time_ms,
                topics,
            },
        )
    }
}

impl FromBytes for CreatableTopicResultKey {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        CreatableTopicResultKey { name }
    }
}

impl FromBytes for CreatableTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = if version >= 1 {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        CreatableTopicResult {
            error_code,
            error_message,
        }
    }
}
