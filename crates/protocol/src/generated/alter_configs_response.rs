use super::super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlterConfigsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses for each resource.
    pub resources: Vec<AlterConfigsResourceResponse>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlterConfigsResourceResponse {
    /// The resource error code.
    pub error_code: i16,

    /// The resource error message, or null if there was no error.
    pub error_message: Option<String>,

    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,
}

impl ApiResponse for AlterConfigsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let resources = Vec::<AlterConfigsResourceResponse>::deserialize(version, bytes);
        (
            header,
            AlterConfigsResponse {
                throttle_time_ms,
                resources,
            },
        )
    }
}

impl FromBytes for AlterConfigsResourceResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = i16::deserialize(version, bytes);
        let error_message = Option::<String>::deserialize(version, bytes);
        let resource_type = i8::deserialize(version, bytes);
        let resource_name = String::deserialize(version, bytes);
        AlterConfigsResourceResponse {
            error_code,
            error_message,
            resource_type,
            resource_name,
        }
    }
}
