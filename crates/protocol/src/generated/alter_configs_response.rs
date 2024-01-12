use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AlterConfigsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses for each resource.
    pub resources: Vec<AlterConfigsResourceResponse>,
}

#[derive(Clone, Debug, Default)]
pub struct AlterConfigsResourceResponse {
    /// The resource error code.
    pub error_code: i16,

    /// The resource error message, or null if there was no error.
    pub error_message: String,

    /// The resource type.
    pub resource_type: i8,

    /// The resource name.
    pub resource_name: String,
}

impl ApiResponse for AlterConfigsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resources = if version >= 0 {
            Vec::<AlterConfigsResourceResponse>::deserialize(version, bytes)
        } else {
            Default::default()
        };
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
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_message = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resource_type = if version >= 0 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resource_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        AlterConfigsResourceResponse {
            error_code,
            error_message,
            resource_type,
            resource_name,
        }
    }
}
