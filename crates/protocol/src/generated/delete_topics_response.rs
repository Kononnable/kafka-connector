use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DeleteTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The results for each topic.
    pub responses: Vec<DeletableTopicResult>,
}

#[derive(Debug, Clone)]
pub struct DeletableTopicResult {
    /// The topic name
    pub name: String,

    /// The deletion error, or 0 if the deletion succeeded.
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

impl Default for DeleteTopicsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            responses: Default::default(),
        }
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

impl Default for DeletableTopicResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            error_code: Default::default(),
        }
    }
}
