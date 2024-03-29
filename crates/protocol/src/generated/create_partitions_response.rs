use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreatePartitionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The partition creation results for each topic.
    pub results: Vec<CreatePartitionsTopicResult>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatePartitionsTopicResult {
    /// The topic name.
    pub name: String,

    /// The result error, or zero if there was no error.
    pub error_code: i16,

    /// The result message, or null if there was no error.
    pub error_message: Option<String>,
}

impl ApiResponse for CreatePartitionsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<CreatePartitionsTopicResult>::deserialize(version, bytes);
        (
            header,
            CreatePartitionsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for CreatePartitionsTopicResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let name = String::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        CreatePartitionsTopicResult {
            name,
            error_code,
            error_message,
        }
    }
}
