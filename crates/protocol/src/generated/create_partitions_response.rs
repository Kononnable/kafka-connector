use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreatePartitionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The partition creation results for each topic.
    pub results: Vec<CreatePartitionsTopicResult>,
}

#[derive(Clone, Debug, Default)]
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
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<CreatePartitionsTopicResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
        let error_message = if version >= 0 {
            Option::<String>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        CreatePartitionsTopicResult {
            name,
            error_code,
            error_message,
        }
    }
}
